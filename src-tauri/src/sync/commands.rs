use std::sync::Arc;
use std::time::Instant;
use tauri::{AppHandle, Emitter, State};

use super::discovery;
use super::identity;
use super::transport;
use super::types::{FoundPeer, PeerInfo, SyncResult, SyncStatus};
use crate::db::Database;

// ============================================================
// Состояние синка
// ============================================================

pub struct SyncState {
    pub listener_port: u16,
    pub mdns: tokio::sync::Mutex<Option<mdns_sd::ServiceDaemon>>,
    pub status: tokio::sync::Mutex<SyncStatus>,
}

impl SyncState {
    pub fn new(port: u16) -> Self {
        Self {
            listener_port: port,
            mdns: tokio::sync::Mutex::new(None),
            status: tokio::sync::Mutex::new(SyncStatus::Idle {
                last_sync_at: None,
                last_peer_name: None,
            }),
        }
    }

    pub async fn set_status(&self, status: SyncStatus) {
        *self.status.lock().await = status;
    }

    pub async fn get_status(&self) -> SyncStatus {
        self.status.lock().await.clone()
    }
}

// ============================================================
// Служебные команды
// ============================================================

#[tauri::command]
pub fn sync_get_identity(db: State<'_, Arc<Database>>) -> Result<identity::DeviceIdentityPub, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = identity::current_identity(&conn)?;
    Ok(identity::DeviceIdentityPub {
        device_id: id.device_id,
        device_name: id.device_name,
    })
}

#[tauri::command]
pub fn sync_rename_device(db: State<'_, Arc<Database>>, new_name: String) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    identity::rename_device(&conn, &new_name)
}

#[tauri::command]
pub async fn sync_get_status(state: State<'_, SyncState>) -> Result<SyncStatus, String> {
    Ok(state.get_status().await)
}

#[tauri::command]
pub fn sync_get_peers(db: State<'_, Arc<Database>>) -> Result<Vec<PeerInfo>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT device_id, device_name, public_key, last_seen_at
             FROM sync_peers ORDER BY last_seen_at DESC NULLS LAST, paired_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let peers = stmt
        .query_map([], |r| {
            Ok(PeerInfo {
                device_id: r.get(0)?,
                device_name: r.get(1)?,
                public_key: r.get(2)?,
                paired: true,
                last_seen_at: r.get(3)?,
                last_sync_at: None,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(peers)
}

#[tauri::command]
pub fn sync_forget_peer(db: State<'_, Arc<Database>>, device_id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM sync_peers WHERE device_id = ?1", [device_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================
// Главная команда: «СИНК»
// ============================================================

#[tauri::command]
pub async fn sync_now(
    app: AppHandle,
    state: State<'_, SyncState>,
    db: State<'_, Arc<Database>>,
    force_peer_id: Option<String>,
) -> Result<SyncStatus, String> {
    let db_arc: Arc<Database> = Arc::clone(&db);

    state.set_status(SyncStatus::Discovering).await;
    let _ = app.emit("sync:status", &SyncStatus::Discovering);

    let local_identity = {
        let conn = db_arc.conn.lock().map_err(|e| e.to_string())?;
        identity::current_identity(&conn)?
    };

    let discovered = match discovery::browse_peers().await {
        Ok(v) => v,
        Err(e) => {
            let status = SyncStatus::Error { message: e };
            state.set_status(status.clone()).await;
            let _ = app.emit("sync:status", &status);
            return Ok(status);
        }
    };

    let discovered: Vec<discovery::DiscoveredPeer> = discovered
        .into_iter()
        .filter(|p| p.device_id != local_identity.device_id)
        .collect();

    if discovered.is_empty() {
        let status = SyncStatus::Error {
            message: "Устройства не найдены".to_string(),
        };
        state.set_status(status.clone()).await;
        let _ = app.emit("sync:status", &status);
        return Ok(status);
    }

    let paired_ids: Vec<String> = {
        let conn = db_arc.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT device_id FROM sync_peers")
            .map_err(|e| e.to_string())?;
        stmt.query_map([], |r| r.get::<_, String>(0))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?
    };

    let mut known: Vec<discovery::DiscoveredPeer> = Vec::new();
    let mut unknown: Vec<discovery::DiscoveredPeer> = Vec::new();
    for p in discovered {
        if paired_ids.contains(&p.device_id) {
            known.push(p);
        } else {
            unknown.push(p);
        }
    }

    // force_peer_id — синкаемся только с этим
    if let Some(target) = &force_peer_id {
        if let Some(p) = known.iter().find(|x| &x.device_id == target).cloned() {
            known = vec![p];
            unknown.clear();
        } else if let Some(p) = unknown.iter().find(|x| &x.device_id == target).cloned() {
            known = vec![p];
            unknown.clear();
        } else {
            let status = SyncStatus::Error {
                message: format!("Устройство {} не найдено в сети", target),
            };
            state.set_status(status.clone()).await;
            let _ = app.emit("sync:status", &status);
            return Ok(status);
        }
    }

    // Синкаемся с известными
    let mut total_pushed = 0usize;
    let mut total_pulled = 0usize;
    let mut last_peer_name = String::new();
    let mut last_sync_at: Option<String> = None;

    for peer in &known {
        state
            .set_status(SyncStatus::Syncing {
                peer_name: peer.device_name.clone(),
            })
            .await;
        let _ = app.emit(
            "sync:status",
            &SyncStatus::Syncing {
                peer_name: peer.device_name.clone(),
            },
        );

        let started = Instant::now();
        match transport::sync_with_peer(db_arc.clone(), &peer.address, Some(&peer.device_id), false).await {
            Ok(result) => {
                total_pushed += result.pushed;
                total_pulled += result.pulled;
                last_peer_name = result.peer_name.clone();
                last_sync_at = Some(crate::sync::now_sqlite_datetime(&db_arc).unwrap_or_default());
                let _ = app.emit(
                    "sync:peer_done",
                    &SyncResult {
                        peer_name: result.peer_name,
                        pushed: result.pushed,
                        pulled: result.pulled,
                        duration_ms: started.elapsed().as_millis() as u64,
                    },
                );
            }
            Err(e) => {
                eprintln!("[sync] с {} ошибка: {}", peer.device_name, e);
            }
        }
    }

    // Если остались неизвестные — модалка
    if !unknown.is_empty() {
        let found: Vec<FoundPeer> = unknown
            .iter()
            .map(|p| FoundPeer {
                device_id: p.device_id.clone(),
                device_name: p.device_name.clone(),
                address: p.address.clone(),
                paired: false,
            })
            .collect();

        let status = SyncStatus::AwaitingPairing { found };
        state.set_status(status.clone()).await;
        let _ = app.emit("sync:awaiting_pairing", &status);
        return Ok(status);
    }

    let status = if total_pushed + total_pulled > 0 {
        SyncStatus::Done {
            peer_name: last_peer_name,
            pushed: total_pushed,
            pulled: total_pulled,
            duration_ms: 0,
        }
    } else {
        SyncStatus::Idle {
            last_sync_at,
            last_peer_name: if last_peer_name.is_empty() { None } else { Some(last_peer_name) },
        }
    };
    state.set_status(status.clone()).await;
    let _ = app.emit("sync:status", &status);

    Ok(status)
}

/// Пейринг + синк. Вызывается после модалки.
#[tauri::command]
pub async fn sync_pair_and_sync(
    app: AppHandle,
    state: State<'_, SyncState>,
    db: State<'_, Arc<Database>>,
    device_id: String,
    address: String,
) -> Result<SyncStatus, String> {
    let db_arc: Arc<Database> = Arc::clone(&db);

    state
        .set_status(SyncStatus::Syncing {
            peer_name: address.clone(),
        })
        .await;
    let _ = app.emit(
        "sync:status",
        &SyncStatus::Syncing {
            peer_name: address.clone(),
        },
    );

    let started = Instant::now();
    match transport::sync_with_peer(db_arc.clone(), &address, Some(&device_id), true).await {
        Ok(result) => {
            let status = SyncStatus::Done {
                peer_name: result.peer_name,
                pushed: result.pushed,
                pulled: result.pulled,
                duration_ms: started.elapsed().as_millis() as u64,
            };
            state.set_status(status.clone()).await;
            let _ = app.emit("sync:status", &status);
            Ok(status)
        }
        Err(e) => {
            let status = SyncStatus::Error {
                message: e.to_string(),
            };
            state.set_status(status.clone()).await;
            let _ = app.emit("sync:status", &status);
            Ok(status)
        }
    }
}