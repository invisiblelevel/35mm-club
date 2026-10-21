use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use super::crdt;
use super::identity;
use super::types::{SyncError, SyncMessage, SyncRecord, BATCH_SIZE, PROTOCOL_VERSION};
use crate::db::Database;

// ============================================================
// JSON-lines
// ============================================================

async fn send_msg<W: AsyncWriteExt + Unpin>(writer: &mut W, msg: &SyncMessage) -> Result<(), SyncError> {
    let mut json = serde_json::to_string(msg).map_err(|e| SyncError::Internal(e.to_string()))?;
    json.push('\n');
    writer.write_all(json.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

async fn recv_msg<R: AsyncBufReadExt + Unpin>(reader: &mut R) -> Result<SyncMessage, SyncError> {
    let mut line = String::new();
    let n = reader.read_line(&mut line).await?;
    if n == 0 {
        return Err(SyncError::Io("connection closed".to_string()));
    }
    serde_json::from_str(line.trim()).map_err(|e| SyncError::Internal(format!("parse: {}", e)))
}

// ============================================================
// Handshake
// ============================================================

fn make_nonce() -> (String, [u8; 32]) {
    let mut nonce = [0u8; 32];
    OsRng.fill_bytes(&mut nonce);
    (B64.encode(nonce), nonce)
}

fn sign_nonce(nonce: &[u8; 32]) -> Result<String, SyncError> {
    let key = identity::signing_key().map_err(SyncError::Internal)?;
    let sig = key.sign(nonce);
    Ok(B64.encode(sig.to_bytes()))
}

fn verify_nonce(pubkey_b64: &str, nonce: &[u8; 32], sig_b64: &str) -> Result<(), SyncError> {
    let pk_bytes = B64.decode(pubkey_b64).map_err(|_| SyncError::BadSignature)?;
    let pk_arr: [u8; 32] = pk_bytes.try_into().map_err(|_| SyncError::BadSignature)?;
    let vk = VerifyingKey::from_bytes(&pk_arr).map_err(|_| SyncError::BadSignature)?;

    let sig_bytes = B64.decode(sig_b64).map_err(|_| SyncError::BadSignature)?;
    let sig_arr: [u8; 64] = sig_bytes.try_into().map_err(|_| SyncError::BadSignature)?;
    let sig = Signature::from_bytes(&sig_arr);

    vk.verify(nonce, &sig).map_err(|_| SyncError::BadSignature)
}

// ============================================================
// Батчи
// ============================================================

fn collect_records_after(
    conn: &rusqlite::Connection,
    after_seq: i64,
    limit: usize,
) -> Result<Vec<SyncRecord>, SyncError> {
    let mut stmt = conn
        .prepare(
            "SELECT seq, device_id, entity_type, entity_key, action, payload, updated_at
             FROM sync_log WHERE seq > ?1 ORDER BY seq ASC LIMIT ?2",
        )
        .map_err(|e| SyncError::Internal(e.to_string()))?;

    let rows = stmt
        .query_map(rusqlite::params![after_seq, limit as i64], |r| {
            Ok(SyncRecord {
                seq: r.get(0)?,
                device_id: r.get(1)?,
                entity_type: r.get(2)?,
                entity_key: r.get(3)?,
                action: r.get(4)?,
                payload: r.get(5)?,
                updated_at: r.get(6)?,
            })
        })
        .map_err(|e| SyncError::Internal(e.to_string()))?;

    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| SyncError::Internal(e.to_string()))?);
    }
    Ok(out)
}

fn max_seq_upto(records: &[SyncRecord]) -> i64 {
    records.last().map(|r| r.seq).unwrap_or(0)
}

// ============================================================
// Сервер
// ============================================================

pub async fn start_listener(
    db: Arc<Database>,
    local_device_id: String,
    requested_port: u16,
) -> Result<u16, SyncError> {
    let listener = TcpListener::bind(("0.0.0.0", requested_port))
        .await
        .map_err(|e| SyncError::Io(format!("bind {}: {}", requested_port, e)))?;

    let actual_port = listener
        .local_addr()
        .map_err(|e| SyncError::Io(e.to_string()))?
        .port();

    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let db = db.clone();
                    let local_id = local_device_id.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_incoming(stream, db, local_id).await {
                            eprintln!("[sync] incoming from {} error: {}", addr, e);
                        }
                    });
                }
                Err(e) => eprintln!("[sync] accept error: {}", e),
            }
        }
    });

    Ok(actual_port)
}

async fn handle_incoming(
    stream: TcpStream,
    db: Arc<Database>,
    local_device_id: String,
) -> Result<(), SyncError> {
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = writer;

    // 1. HELLO
    let (remote_id, remote_name, remote_pubkey, accept_unknown) = match recv_msg(&mut reader).await? {
        SyncMessage::Hello {
            protocol_version,
            device_id,
            device_name,
            public_key,
            nonce,
            signature,
            accept_unknown,
        } => {
            if protocol_version != PROTOCOL_VERSION {
                let _ = send_msg(
                    &mut writer,
                    &SyncMessage::Bye {
                        success: false,
                        message: Some(format!(
                            "protocol mismatch: remote={} local={}",
                            protocol_version, PROTOCOL_VERSION
                        )),
                    },
                )
                .await;
                return Err(SyncError::ProtocolMismatch {
                    remote: protocol_version,
                    local: PROTOCOL_VERSION,
                });
            }
            if device_id == local_device_id {
                return Err(SyncError::Internal("сам с собой".to_string()));
            }
            let nonce_bytes = B64.decode(&nonce).map_err(|_| SyncError::BadSignature)?;
            let nonce_arr: [u8; 32] = nonce_bytes.try_into().map_err(|_| SyncError::BadSignature)?;
            verify_nonce(&public_key, &nonce_arr, &signature)?;
            (device_id, device_name, public_key, accept_unknown)
        }
        _ => return Err(SyncError::Internal("ожидался HELLO".to_string())),
    };

    // 2. Проверяем пира
    let known_peer = {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        let stored: Option<String> = conn
            .query_row(
                "SELECT public_key FROM sync_peers WHERE device_id = ?1",
                [&remote_id],
                |r| r.get(0),
            )
            .ok();
        match stored {
            Some(pk) if pk == remote_pubkey => true,
            Some(_) => return Err(SyncError::BadSignature),
            None => false,
        }
    };

    // Если пир неизвестен и клиент не просит пейринг — отказ
    if !known_peer && !accept_unknown {
        let _ = send_msg(
            &mut writer,
            &SyncMessage::Bye {
                success: false,
                message: Some("pairing_required".to_string()),
            },
        )
        .await;
        return Err(SyncError::UnknownPeer);
    }

    // 2.5. Если accept_unknown && !known_peer — добавляем пира СЕЙЧАС
    // (после успешного handshake, до синка)
    if !known_peer && accept_unknown {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        conn.execute(
            "INSERT OR IGNORE INTO sync_peers (device_id, device_name, public_key)
             VALUES (?1, ?2, ?3)",
            rusqlite::params![remote_id, remote_name, remote_pubkey],
        )
        .map_err(|e| SyncError::Internal(e.to_string()))?;
    }

    // 3. HELLO_ACK
    let local_identity = {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        identity::current_identity(&conn).map_err(SyncError::Internal)?
    };
    let (ack_nonce, ack_nonce_bytes) = make_nonce();
    let ack_sig = sign_nonce(&ack_nonce_bytes)?;

    send_msg(
        &mut writer,
        &SyncMessage::HelloAck {
            protocol_version: PROTOCOL_VERSION,
            device_id: local_identity.device_id.clone(),
            device_name: local_identity.device_name.clone(),
            public_key: local_identity.public_key.clone(),
            nonce: ack_nonce,
            signature: ack_sig,
            known_peer: known_peer || accept_unknown,
        },
    )
    .await?;

    // 4. READY
    let client_last_pulled = match recv_msg(&mut reader).await? {
        SyncMessage::Ready { last_pulled_seq } => last_pulled_seq,
        _ => return Err(SyncError::Internal("ожидался READY".to_string())),
    };

    let our_last_pulled = {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        conn.query_row(
            "SELECT last_pulled_seq FROM sync_peers WHERE device_id = ?1",
            [&remote_id],
            |r| r.get::<_, i64>(0),
        )
        .unwrap_or(0)
    };

    send_msg(
        &mut writer,
        &SyncMessage::ReadyAck {
            last_pulled_seq: our_last_pulled,
        },
    )
    .await?;

    // 5. PUSH: свои записи клиенту
    let mut cursor = client_last_pulled;
    loop {
        let records = {
            let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
            collect_records_after(&conn, cursor, BATCH_SIZE).map_err(|e| SyncError::Internal(e.to_string()))?
        };
        let last = records.len() < BATCH_SIZE;
        let next_cursor = max_seq_upto(&records);

        send_msg(&mut writer, &SyncMessage::Batch { records, last }).await?;

        match recv_msg(&mut reader).await? {
            SyncMessage::BatchAck { .. } => {}
            _ => return Err(SyncError::Internal("ожидался BatchAck".to_string())),
        }

        cursor = next_cursor;
        if last {
            break;
        }
    }

    // 6. PULL: получаем от клиента
    let mut applied_max_seq = our_last_pulled;
    loop {
        match recv_msg(&mut reader).await? {
            SyncMessage::Batch { records, last } => {
                let mut max_seq = applied_max_seq;
                {
                    let mut conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
                    let tx = conn.transaction().map_err(|e| SyncError::Internal(e.to_string()))?;

                    for rec in &records {
                        if rec.seq > max_seq {
                            max_seq = rec.seq;
                        }
                        if rec.device_id == local_identity.device_id {
                            continue;
                        }
                        let _ = crdt::apply_remote_change(&tx, rec)?;
                    }

                    tx.execute(
                        "UPDATE sync_peers SET last_pulled_seq = ?1, last_seen_at = datetime('now')
                         WHERE device_id = ?2",
                        rusqlite::params![max_seq, remote_id],
                    )
                    .map_err(|e| SyncError::Internal(e.to_string()))?;

                    tx.commit().map_err(|e| SyncError::Internal(e.to_string()))?;
                    // tx уничтожен здесь — conn можно отпускать
                }

                applied_max_seq = max_seq;

                send_msg(
                    &mut writer,
                    &SyncMessage::BatchAck {
                        applied_seq: applied_max_seq,
                    },
                )
                .await?;

                if last {
                    break;
                }
            }
            _ => return Err(SyncError::Internal("ожидался Batch".to_string())),
        }
    }

    // 7. Обновляем last_pushed_seq
    {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        conn.execute(
            "UPDATE sync_peers SET last_pushed_seq = ?1, last_seen_at = datetime('now')
             WHERE device_id = ?2",
            rusqlite::params![cursor, remote_id],
        )
        .map_err(|e| SyncError::Internal(e.to_string()))?;
    }

    send_msg(
        &mut writer,
        &SyncMessage::Bye {
            success: true,
            message: None,
        },
    )
    .await?;

    Ok(())
}

// ============================================================
// Клиент
// ============================================================

pub struct ClientSyncResult {
    pub pushed: usize,
    pub pulled: usize,
    pub known_peer: bool,
    pub peer_name: String,
}

pub async fn sync_with_peer(
    db: Arc<Database>,
    address: &str,
    expected_peer_id: Option<&str>,
    force_pair: bool,
) -> Result<ClientSyncResult, SyncError> {
    let stream = TcpStream::connect(address)
        .await
        .map_err(|e| SyncError::Io(format!("connect {}: {}", address, e)))?;
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = writer;

    let local_identity = {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        identity::current_identity(&conn).map_err(SyncError::Internal)?
    };

    // 1. HELLO
    let (nonce, nonce_bytes) = make_nonce();
    let sig = sign_nonce(&nonce_bytes)?;

    send_msg(
        &mut writer,
        &SyncMessage::Hello {
            protocol_version: PROTOCOL_VERSION,
            device_id: local_identity.device_id.clone(),
            device_name: local_identity.device_name.clone(),
            public_key: local_identity.public_key.clone(),
            nonce,
            signature: sig,
            accept_unknown: force_pair,
        },
    )
    .await?;

    // 2. HELLO_ACK
    let (remote_id, remote_name, remote_pubkey, known_peer) = match recv_msg(&mut reader).await? {
        SyncMessage::HelloAck {
            protocol_version,
            device_id,
            device_name,
            public_key,
            nonce,
            signature,
            known_peer,
        } => {
            if protocol_version != PROTOCOL_VERSION {
                return Err(SyncError::ProtocolMismatch {
                    remote: protocol_version,
                    local: PROTOCOL_VERSION,
                });
            }
            if let Some(expected) = expected_peer_id {
                if expected != device_id {
                    return Err(SyncError::Internal(format!(
                        "ожидали пира {}, получили {}",
                        expected, device_id
                    )));
                }
            }
            let nonce_bytes = B64.decode(&nonce).map_err(|_| SyncError::BadSignature)?;
            let nonce_arr: [u8; 32] = nonce_bytes.try_into().map_err(|_| SyncError::BadSignature)?;
            verify_nonce(&public_key, &nonce_arr, &signature)?;
            (device_id, device_name, public_key, known_peer)
        }
        SyncMessage::Bye { message, .. } => {
            let msg = message.unwrap_or("bye".to_string());
            if msg == "pairing_required" {
                return Err(SyncError::UnknownPeer);
            }
            return Err(SyncError::Io(msg));
        }
        _ => return Err(SyncError::Internal("ожидался HELLO_ACK".to_string())),
    };

    // 3. Проверяем pubkey
    let stored_pubkey: Option<String> = {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        conn.query_row(
            "SELECT public_key FROM sync_peers WHERE device_id = ?1",
            [&remote_id],
            |r| r.get(0),
        )
        .ok()
    };

    if let Some(stored) = &stored_pubkey {
        if stored != &remote_pubkey {
            return Err(SyncError::BadSignature);
        }
    }

    if !known_peer && !force_pair {
        let _ = send_msg(
            &mut writer,
            &SyncMessage::Bye {
                success: false,
                message: Some("pairing_required".to_string()),
            },
        )
        .await;
        return Err(SyncError::UnknownPeer);
    }

    // 4. Если force_pair — добавляем пира локально
    if force_pair && stored_pubkey.is_none() {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        conn.execute(
            "INSERT OR IGNORE INTO sync_peers (device_id, device_name, public_key)
             VALUES (?1, ?2, ?3)",
            rusqlite::params![remote_id, remote_name, remote_pubkey],
        )
        .map_err(|e| SyncError::Internal(e.to_string()))?;
    }

    // 5. READY
    let our_last_pulled = {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        conn.query_row(
            "SELECT last_pulled_seq FROM sync_peers WHERE device_id = ?1",
            [&remote_id],
            |r| r.get::<_, i64>(0),
        )
        .unwrap_or(0)
    };

    send_msg(&mut writer, &SyncMessage::Ready { last_pulled_seq: our_last_pulled }).await?;

    // 6. READY_ACK
    let remote_last_pulled = match recv_msg(&mut reader).await? {
        SyncMessage::ReadyAck { last_pulled_seq } => last_pulled_seq,
        _ => return Err(SyncError::Internal("ожидался READY_ACK".to_string())),
    };

    // 7. PULL: получаем от сервера
    let mut pulled_count = 0usize;
    let mut applied_max_seq = our_last_pulled;
    loop {
        match recv_msg(&mut reader).await? {
            SyncMessage::Batch { records, last } => {
                let mut max_seq = applied_max_seq;
                {
                    let mut conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
                    let tx = conn.transaction().map_err(|e| SyncError::Internal(e.to_string()))?;

                    for rec in &records {
                        if rec.seq > max_seq {
                            max_seq = rec.seq;
                        }
                        if rec.device_id == local_identity.device_id {
                            continue;
                        }
                        let _ = crdt::apply_remote_change(&tx, rec)?;
                    }

                    tx.execute(
                        "UPDATE sync_peers SET last_pulled_seq = ?1, last_seen_at = datetime('now')
                         WHERE device_id = ?2",
                        rusqlite::params![max_seq, remote_id],
                    )
                    .map_err(|e| SyncError::Internal(e.to_string()))?;

                    tx.commit().map_err(|e| SyncError::Internal(e.to_string()))?;
                }

                applied_max_seq = max_seq;
                pulled_count += records.len();

                send_msg(&mut writer, &SyncMessage::BatchAck { applied_seq: applied_max_seq }).await?;

                if last {
                    break;
                }
            }
            SyncMessage::Bye { message, .. } => {
                return Err(SyncError::Io(message.unwrap_or("bye".to_string())));
            }
            _ => return Err(SyncError::Internal("ожидался Batch".to_string())),
        }
    }

    // 8. PUSH: отправляем серверу
    let mut pushed_count = 0usize;
    let mut cursor = remote_last_pulled;
    loop {
        let records = {
            let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
            collect_records_after(&conn, cursor, BATCH_SIZE).map_err(|e| SyncError::Internal(e.to_string()))?
        };
        if records.is_empty() {
            send_msg(&mut writer, &SyncMessage::Batch { records: vec![], last: true }).await?;
            let _ = recv_msg(&mut reader).await?;
            break;
        }
        let last = records.len() < BATCH_SIZE;
        let next_cursor = max_seq_upto(&records);
        pushed_count += records.len();

        send_msg(&mut writer, &SyncMessage::Batch { records, last }).await?;

        match recv_msg(&mut reader).await? {
            SyncMessage::BatchAck { .. } => {}
            _ => return Err(SyncError::Internal("ожидался BatchAck".to_string())),
        }

        cursor = next_cursor;
        if last {
            break;
        }
    }

    // 9. last_pushed_seq
    {
        let conn = db.conn.lock().map_err(|e| SyncError::Internal(e.to_string()))?;
        conn.execute(
            "UPDATE sync_peers SET last_pushed_seq = ?1, last_seen_at = datetime('now')
             WHERE device_id = ?2",
            rusqlite::params![cursor, remote_id],
        )
        .map_err(|e| SyncError::Internal(e.to_string()))?;
    }

    let _ = recv_msg(&mut reader).await; // Bye

    Ok(ClientSyncResult {
        pushed: pushed_count,
        pulled: pulled_count,
        known_peer,
        peer_name: remote_name,
    })
}