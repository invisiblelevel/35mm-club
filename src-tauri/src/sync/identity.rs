use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use super::types::DeviceIdentity;

/// Имя файла с приватным ключом (рядом с БД).
const KEY_FILE: &str = "device.key";

/// Кэш в памяти: ключ читается с диска один раз.
static SIGNING_KEY: OnceLock<SigningKey> = OnceLock::new();

/// Инициализирует идентичность устройства.
/// - Если файла `device.key` нет — генерит новый keypair, пишет файл.
/// - Возвращает `DeviceIdentity` (device_id, name, pubkey base64).
/// - Гарантирует, что `device_identity` в БД заполнена и совпадает с ключом.
pub fn init_identity(db_dir: &Path, conn: &rusqlite::Connection) -> Result<DeviceIdentity, String> {
    let key_path = db_dir.join(KEY_FILE);

    let signing_key = if key_path.exists() {
        load_key(&key_path)?
    } else {
        let key = SigningKey::generate(&mut OsRng);
        save_key(&key_path, &key)?;
        key
    };

    let verifying_key = signing_key.verifying_key();
    let pubkey_b64 = B64.encode(verifying_key.to_bytes());
    let device_id = short_device_id(&verifying_key);

    // Кэшируем в память
    let _ = SIGNING_KEY.set(signing_key);

    // Читаем/создаём запись в БД
    let existing: Option<(String, String)> = conn
        .query_row(
            "SELECT device_id, public_key FROM device_identity WHERE id = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .ok();

    let device_name = match existing {
        Some((ref did, ref pkey)) if did == &device_id && pkey == &pubkey_b64 => {
            // Всё совпадает — читаем имя из БД
            conn.query_row(
                "SELECT device_name FROM device_identity WHERE id = 1",
                [],
                |r| r.get::<_, String>(0),
            )
            .unwrap_or_else(|_| default_device_name())
        }
        _ => {
            // Либо пусто, либо ключ сменился — перезаписываем.
            let name = default_device_name();
            conn.execute(
                "INSERT INTO device_identity (id, device_id, device_name, public_key)
                 VALUES (1, ?1, ?2, ?3)
                 ON CONFLICT(id) DO UPDATE SET
                    device_id = excluded.device_id,
                    device_name = excluded.device_name,
                    public_key = excluded.public_key",
                rusqlite::params![device_id, name, pubkey_b64],
            )
            .map_err(|e| e.to_string())?;
            name
        }
    };

    Ok(DeviceIdentity {
        device_id,
        device_name,
        public_key: pubkey_b64,
    })
}

/// Возвращает текущий signing key (после `init_identity`).
pub fn signing_key() -> Result<&'static SigningKey, String> {
    SIGNING_KEY
        .get()
        .ok_or_else(|| "identity не инициализирована".to_string())
}

/// Возвращает identity из БД (без инициализации — только чтение).
pub fn current_identity(conn: &rusqlite::Connection) -> Result<DeviceIdentity, String> {
    conn.query_row(
        "SELECT device_id, device_name, public_key FROM device_identity WHERE id = 1",
        [],
        |r| {
            Ok(DeviceIdentity {
                device_id: r.get(0)?,
                device_name: r.get(1)?,
                public_key: r.get(2)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// Меняет device_name. Возвращает новое имя.
pub fn rename_device(conn: &rusqlite::Connection, new_name: &str) -> Result<String, String> {
    let trimmed = new_name.trim();
    if trimmed.is_empty() {
        return Err("Имя не может быть пустым".to_string());
    }
    if trimmed.chars().count() > 64 {
        return Err("Имя слишком длинное (макс. 64 символа)".to_string());
    }
    conn.execute(
        "UPDATE device_identity SET device_name = ?1 WHERE id = 1",
        [trimmed],
    )
    .map_err(|e| e.to_string())?;
    Ok(trimmed.to_string())
}

// ============================================================
// Вспомогательное
// ============================================================

fn load_key(path: &Path) -> Result<SigningKey, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("read device.key: {}", e))?;
    let bytes = B64
        .decode(raw.trim())
        .map_err(|e| format!("decode device.key: {}", e))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| "device.key: неверная длина ключа".to_string())?;
    Ok(SigningKey::from_bytes(&arr))
}

fn save_key(path: &Path, key: &SigningKey) -> Result<(), String> {
    let b64 = B64.encode(key.to_bytes());
    std::fs::write(path, b64).map_err(|e| format!("write device.key: {}", e))?;
    // На Unix — выставим права 0600
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

/// device_id = первые 16 hex-символов SHA-256(pubkey).
fn short_device_id(vk: &VerifyingKey) -> String {
    let mut hasher = Sha256::new();
    hasher.update(vk.to_bytes());
    let hash = hasher.finalize();
    hex::encode(&hash[..8])
}

/// Имя устройства по умолчанию: hostname ОС, fallback — "35mm Club".
fn default_device_name() -> String {
    if let Ok(host) = std::env::var("COMPUTERNAME") {
        if !host.trim().is_empty() {
            return host;
        }
    }
    if let Ok(host) = std::env::var("HOSTNAME") {
        if !host.trim().is_empty() {
            return host;
        }
    }
    // На Android хоста нет — просто "35mm Club"
    "35mm Club".to_string()
}

/// Путь к файлу ключа (для логов/дебага).
pub fn key_path(db_dir: &Path) -> PathBuf {
    db_dir.join(KEY_FILE)
}
/// Публичная версия DeviceIdentity для фронта (без pubkey — он не нужен юзеру).
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceIdentityPub {
    pub device_id: String,
    pub device_name: String,
}