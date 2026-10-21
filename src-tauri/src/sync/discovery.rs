use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use super::types::{BASE_PORT, DISCOVERY_TIMEOUT_MS, MDNS_SERVICE, PORT_SEARCH_RANGE};

/// Найденное в сети устройство.
#[derive(Debug, Clone)]
pub struct DiscoveredPeer {
    pub device_id: String,
    pub device_name: String,
    pub address: String, // "192.168.1.5:47820"
}

/// Запускает mDNS-регистрацию нашего сервиса.
/// Возвращает порт, на котором реально зарегистрировались (базовый или следующий свободный).
pub async fn register_service(
    device_id: &str,
    device_name: &str,
    tcp_port: u16,
) -> Result<ServiceDaemon, String> {
    let mdns = ServiceDaemon::new().map_err(|e| format!("mdns init: {}", e))?;

    let instance_name = format!("35mmclub-{}", device_id);

    // TXT-записи: device_id, name, version
    let mut props: HashMap<String, String> = HashMap::new();
    props.insert("device_id".to_string(), device_id.to_string());
    props.insert("device_name".to_string(), device_name.to_string());
    props.insert("version".to_string(), super::types::PROTOCOL_VERSION.to_string());
    props.insert("port".to_string(), tcp_port.to_string());

    let host_name = format!("{}.local.", instance_name);

    let info = ServiceInfo::new(
        MDNS_SERVICE,
        &instance_name,
        &host_name,
        "",
        tcp_port,
        props,
    )
    .map_err(|e| format!("mdns ServiceInfo: {}", e))?
    .enable_addr_auto();

    mdns.register(info)
        .map_err(|e| format!("mdns register: {}", e))?;

    Ok(mdns)
}

/// Одноразовый browse: ищет устройства `DISCOVERY_TIMEOUT_MS` мс.
/// Возвращает список найденных (без фильтра по device_id — фильтр на вызывающей стороне).
pub async fn browse_peers() -> Result<Vec<DiscoveredPeer>, String> {
    let mdns = ServiceDaemon::new().map_err(|e| format!("mdns init: {}", e))?;
    let receiver = mdns
        .browse(MDNS_SERVICE)
        .map_err(|e| format!("mdns browse: {}", e))?;

    let found: Arc<Mutex<HashMap<String, DiscoveredPeer>>> = Arc::new(Mutex::new(HashMap::new()));
    let found_clone = found.clone();

    let browse_task = tokio::spawn(async move {
        let deadline = Instant::now() + Duration::from_millis(DISCOVERY_TIMEOUT_MS);
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }
            match tokio::time::timeout(remaining, receiver.recv_async()).await {
                Ok(Ok(event)) => {
                    if let ServiceEvent::ServiceResolved(info) = event {
                        if let Some(peer) = peer_from_service_info(&info) {
                            let mut map = found_clone.lock().await;
                            map.insert(peer.device_id.clone(), peer);
                        }
                    }
                }
                _ => break,
            }
        }
    });

    let _ = browse_task.await;
    let _ = mdns.shutdown();

    let map = found.lock().await;
    let mut result: Vec<DiscoveredPeer> = map.values().cloned().collect();
    result.sort_by(|a, b| a.device_name.cmp(&b.device_name));
    Ok(result)
}

fn peer_from_service_info(info: &ServiceInfo) -> Option<DiscoveredPeer> {
    let device_id = info.get_property_val_str("device_id")?.to_string();
    let device_name = info
        .get_property_val_str("device_name")
        .unwrap_or("35mm Club")
        .to_string();

    // Ищем IPv4-адрес
    let addresses = info.get_addresses();
    let ipv4 = addresses.iter().find(|ip| ip.is_ipv4())?;
    let port = info.get_port();
    let address = format!("{}:{}", ipv4, port);

    Some(DiscoveredPeer {
        device_id,
        device_name,
        address,
    })
}

/// Ищет свободный TCP-порт начиная с BASE_PORT.
/// Возвращает порт, который можно занять (проверяет bind).
pub fn find_free_port() -> Result<u16, String> {
    for offset in 0..PORT_SEARCH_RANGE {
        let port = BASE_PORT + offset;
        match std::net::TcpListener::bind(("0.0.0.0", port)) {
            Ok(listener) => {
                drop(listener);
                return Ok(port);
            }
            Err(_) => continue,
        }
    }
    Err(format!(
        "не удалось найти свободный порт в диапазоне {}-{}",
        BASE_PORT,
        BASE_PORT + PORT_SEARCH_RANGE
    ))
}