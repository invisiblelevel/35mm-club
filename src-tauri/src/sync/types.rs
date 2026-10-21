use serde::{Deserialize, Serialize};

/// Версия протокола. Меняем при несовместимых изменениях.
pub const PROTOCOL_VERSION: u32 = 1;

/// Размер батча при передаче изменений.
pub const BATCH_SIZE: usize = 200;

/// Сколько ждём mDNS при поиске (мс).
pub const DISCOVERY_TIMEOUT_MS: u64 = 3000;

/// Базовый TCP-порт. Если занят — ищем следующий до +10.
pub const BASE_PORT: u16 = 47820;
pub const PORT_SEARCH_RANGE: u16 = 10;

/// mDNS-сервис.
pub const MDNS_SERVICE: &str = "_35mmclub._tcp.local.";

// ============================================================
// Идентичность
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub device_id: String,
    pub device_name: String,
    pub public_key: String, // base64
}

// ============================================================
// Пир
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub device_id: String,
    pub device_name: String,
    pub public_key: String,
    pub paired: bool,
    pub last_seen_at: Option<String>,
    pub last_sync_at: Option<String>,
}

/// Найденное в сети устройство (ещё не спаренное или спаренное).
#[derive(Debug, Clone, Serialize)]
pub struct FoundPeer {
    pub device_id: String,
    pub device_name: String,
    pub address: String, // "192.168.1.5:47820"
    pub paired: bool,
}

// ============================================================
// Статус синка (для UI)
// ============================================================

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SyncStatus {
    /// Ничего не происходит.
    Idle {
        last_sync_at: Option<String>,
        last_peer_name: Option<String>,
    },
    /// Идёт поиск устройств.
    Discovering,
    /// Найдены устройства, ждём подтверждения юзера.
    AwaitingPairing { found: Vec<FoundPeer> },
    /// Идёт обмен с конкретным пиром.
    Syncing { peer_name: String },
    /// Успешно синхронизировано.
    Done {
        peer_name: String,
        pushed: usize,
        pulled: usize,
        duration_ms: u64,
    },
    /// Ошибка.
    Error { message: String },
}

// ============================================================
// Протокол сообщений (JSON-lines по TCP)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SyncMessage {
    /// Клиент → сервер: начало handshake.
    Hello {
        protocol_version: u32,
        device_id: String,
        device_name: String,
        public_key: String, // base64
        nonce: String,      // base64, 32 bytes
        signature: String,  // base64, подпись nonce приватным ключом
        /// true, если клиент готов на TOFU-пейринг (юзер нажал «Подключить»)
        accept_unknown: bool,
    },
    /// Сервер → клиент: ответный handshake.
    HelloAck {
        protocol_version: u32,
        device_id: String,
        device_name: String,
        public_key: String,
        nonce: String,
        signature: String,
        /// true, если сервер уже знает этого клиента в sync_peers.
        known_peer: bool,
    },
    /// Клиент → сервер: «готов принять». Передаёт свои last_pulled_seq.
    Ready {
        last_pulled_seq: i64, // какой seq ОТ ЭТОГО пира мы уже применили
    },
    /// Сервер → клиент: «готов», передаёт свои last_pulled_seq.
    ReadyAck {
        last_pulled_seq: i64, // какой seq ОТ ЭТОГО клиента сервер уже применил
    },
    /// Одна сторона → другая: батч изменений.
    Batch {
        records: Vec<SyncRecord>,
        /// true, если это последний батч.
        last: bool,
    },
    /// Подтверждение получения батча.
    BatchAck {
        /// До какого seq применили.
        applied_seq: i64,
    },
    /// Всё, синк завершён.
    Bye {
        success: bool,
        message: Option<String>,
    },
}

// ============================================================
// Запись в sync_log (для передачи по сети)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    pub seq: i64,
    pub device_id: String,
    pub entity_type: String,
    pub entity_key: String,
    pub action: String, // "upsert" | "delete"
    pub payload: Option<String>,
    pub updated_at: String,
}

// ============================================================
// Результат синка (для команды sync_now)
// ============================================================

#[derive(Debug, Clone, Serialize)]
pub struct SyncResult {
    pub peer_name: String,
    pub pushed: usize,
    pub pulled: usize,
    pub duration_ms: u64,
}

// ============================================================
// Ошибки handshake
// ============================================================

#[derive(Debug)]
pub enum SyncError {
    ProtocolMismatch { remote: u32, local: u32 },
    UnknownPeer,
    BadSignature,
    Io(String),
    Internal(String),
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::ProtocolMismatch { remote, local } => {
                write!(f, "Несовместимая версия протокола: {} (у нас {})", remote, local)
            }
            SyncError::UnknownPeer => write!(f, "Устройство не спарено"),
            SyncError::BadSignature => write!(f, "Неверная подпись handshake"),
            SyncError::Io(s) => write!(f, "Сетевая ошибка: {}", s),
            SyncError::Internal(s) => write!(f, "Внутренняя ошибка: {}", s),
        }
    }
}

impl std::error::Error for SyncError {}

impl From<std::io::Error> for SyncError {
    fn from(e: std::io::Error) -> Self {
        SyncError::Io(e.to_string())
    }
}
impl From<String> for SyncError {
    fn from(s: String) -> Self {
        SyncError::Internal(s)
    }
}