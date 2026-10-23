use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const CACHE_TTL_DAYS: i64 = 30;

pub fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..8])
}

pub fn covers_dir(app_dir: &Path) -> PathBuf {
    app_dir.join("covers")
}

pub fn get_cached(conn: &Connection, url: &str) -> Option<String> {
    let hash = hash_url(url);
    let local_path: Option<String> = conn.query_row(
        "SELECT local_path FROM cover_cache WHERE hash = ?1",
        [&hash],
        |r| r.get(0),
    ).ok();

    if let Some(path) = local_path {
        if Path::new(&path).exists() {
            return Some(path);
        }
        let _ = conn.execute("DELETE FROM cover_cache WHERE hash = ?1", [&hash]);
    }
    None
}

/// Определяет MIME-тип по магическим байтам. Возвращает расширение файла и MIME.
fn detect_image_type(bytes: &[u8]) -> Option<(&'static str, &'static str)> {
    if bytes.len() < 12 {
        return None;
    }
    // JPEG: FF D8 FF
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some(("jpg", "image/jpeg"));
    }
    // PNG: 89 50 4E 47 0D 0A 1A 0A
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some(("png", "image/png"));
    }
    // WebP: RIFF....WEBP
    if &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Some(("webp", "image/webp"));
    }
    // GIF: GIF8
    if bytes.starts_with(&[0x47, 0x49, 0x46, 0x38]) {
        return Some(("gif", "image/gif"));
    }
    // AVIF/HEIF: ....ftyp
    if bytes.len() >= 12 && &bytes[4..8] == b"ftyp" {
        let brand = &bytes[8..12];
        if brand == b"avif" || brand == b"avis" {
            return Some(("avif", "image/avif"));
        }
    }
    None
}

pub fn image_mime_for_path(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    if lower.ends_with(".png") { "image/png" }
    else if lower.ends_with(".webp") { "image/webp" }
    else if lower.ends_with(".gif") { "image/gif" }
    else if lower.ends_with(".avif") { "image/avif" }
    else { "image/jpeg" }
}

/// Скачивает байты обложки. Без БД, без Connection — можно await-ить.
pub async fn download_bytes(url: &str, referer: &str) -> Result<Vec<u8>, String> {
    let client = reqwest::Client::builder()
        .user_agent("35mmClub/1.2.0 (https://github.com/invisiblelevel/35mm-club)")
        .build()
        .map_err(|e| e.to_string())?;

    let mut req = client
        .get(url)
        .header("Accept", "image/avif,image/webp,image/png,image/jpeg,image/*;q=0.8,*/*;q=0.5");
    if !referer.is_empty() {
        req = req.header("Referer", referer);
    }

    let resp = req.send().await.map_err(|e| format!("download error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}: {}", resp.status(), url));
    }

    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    let bytes = bytes.to_vec();

    // Проверяем, что это действительно картинка (а не HTML анти-бота / 404-страница)
    if detect_image_type(&bytes).is_none() {
        let preview: String = bytes.iter().take(80)
            .map(|b| if b.is_ascii_graphic() || *b == b' ' { *b as char } else { '.' })
            .collect();
        return Err(format!(
            "Not an image from {}: {} bytes, preview: {:?}",
            url, bytes.len(), preview
        ));
    }

    Ok(bytes)
}

/// Сохраняет байты в файл и записывает в cover_cache. Sync — без await.
pub fn save_to_cache(
    app_dir: &Path,
    conn: &Connection,
    url: &str,
    bytes: Vec<u8>,
) -> Result<String, String> {
    // Уже в кэше? (на случай гонки)
    if let Some(path) = get_cached(conn, url) {
        return Ok(path);
    }

    // Валидация: только реальные картинки
    let (ext, _mime) = detect_image_type(&bytes)
        .ok_or_else(|| format!("Not an image ({} bytes) from {}", bytes.len(), url))?;

    let hash = hash_url(url);
    let dir = covers_dir(app_dir);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let local_path = dir.join(format!("{}.{}", hash, ext));
    let size = bytes.len() as i64;

    std::fs::write(&local_path, &bytes).map_err(|e| e.to_string())?;

    let path_str = local_path.to_string_lossy().to_string();
    conn.execute(
        "INSERT INTO cover_cache (hash, url, local_path, size, cached_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))
         ON CONFLICT(hash) DO UPDATE SET
            url = excluded.url,
            local_path = excluded.local_path,
            size = excluded.size,
            cached_at = excluded.cached_at",
        rusqlite::params![hash, url, path_str, size],
    ).map_err(|e| e.to_string())?;

    Ok(path_str)
}

pub fn cleanup(conn: &Connection, app_dir: &Path, force: bool) -> Result<usize, String> {
    let mut stmt = conn.prepare(
        "SELECT poster_path FROM titles WHERE poster_path IS NOT NULL"
    ).map_err(|e| e.to_string())?;
    let used_urls: Vec<String> = stmt.query_map([], |r| r.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let used_set: std::collections::HashSet<String> = used_urls.into_iter().collect();

    let mut stmt = conn.prepare(
        "SELECT hash, url, local_path, cached_at FROM cover_cache"
    ).map_err(|e| e.to_string())?;
    let rows: Vec<(String, String, String, String)> = stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
    }).map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().naive_utc();
    let mut removed = 0;

    for (hash, url, local_path, cached_at_str) in rows {
        let is_used = used_set.contains(&url);
        let is_old = if force {
            true
        } else {
            match chrono::NaiveDateTime::parse_from_str(&cached_at_str, "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => (now - dt).num_days() >= CACHE_TTL_DAYS,
                Err(_) => true,
            }
        };

        if !is_used && is_old {
            let _ = std::fs::remove_file(&local_path);
            let _ = conn.execute("DELETE FROM cover_cache WHERE hash = ?1", [&hash]);
            removed += 1;
        }
    }

    let dir = covers_dir(app_dir);
    if dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            let known: std::collections::HashSet<String> = {
                let mut s = std::collections::HashSet::new();
                let mut stmt = conn.prepare("SELECT local_path FROM cover_cache").map_err(|e| e.to_string())?;
                let paths: Vec<String> = stmt.query_map([], |r| r.get(0))
                    .map_err(|e| e.to_string())?
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.to_string())?;
                for p in paths { s.insert(p); }
                s
            };

            for entry in entries.flatten() {
                let path = entry.path();
                let path_str = path.to_string_lossy().to_string();
                if !known.contains(&path_str) {
                    let ext_ok = path.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| matches!(e.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif" | "avif"))
                        .unwrap_or(false);
                    if ext_ok {
                        let _ = std::fs::remove_file(&path);
                        removed += 1;
                    }
                }
            }
        }
    }

    Ok(removed)
}

pub fn should_cleanup(conn: &Connection) -> bool {
    let oldest: Option<String> = conn.query_row(
        "SELECT MIN(cached_at) FROM cover_cache", [], |r| r.get(0)
    ).ok().flatten();

    match oldest {
        None => false,
        Some(dt_str) => match chrono::NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => (chrono::Utc::now().naive_utc() - dt).num_days() >= CACHE_TTL_DAYS,
            Err(_) => false,
        },
    }
}