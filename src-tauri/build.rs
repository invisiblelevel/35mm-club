use std::fs;
use std::path::PathBuf;

fn main() {
    // ==========================================
    // Читаем .env из корня проекта и прокидываем
    // переменные в option_env!/env! на этапе компиляции.
    //
    // .env в .gitignore — в репу не попадёт.
    // Шаблон: .env.example в корне проекта.
    // ==========================================
    load_env();

    tauri_build::build()
}

fn load_env() {
    // build.rs лежит в src-tauri/, корень проекта — на уровень выше
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
    let project_root = match manifest_dir.parent() {
        Some(p) => p.to_path_buf(),
        None => {
            println!("cargo:warning=[env] не удалось определить корень проекта");
            return;
        }
    };

    // Пробуем два места: корень проекта и src-tauri/
    let candidates = [
        project_root.join(".env"),
        manifest_dir.join(".env"),
    ];

    let mut loaded_any = false;

    for path in candidates.iter() {
        if !path.exists() {
            continue;
        }
        let Ok(content) = fs::read_to_string(path) else {
            println!("cargo:warning=[env] не удалось прочитать {}", path.display());
            continue;
        };

        println!("cargo:warning=[env] читаю {}", path.display());

        for line in content.lines() {
            let line = line.trim();
            // пропускаем пустые и комментарии
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // поддержка "export KEY=VAL" и "KEY=VAL"
            let line = line.strip_prefix("export ").unwrap_or(line);
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if key.is_empty() {
                continue;
            }

            // Если переменная уже задана в окружении — не перезаписываем
            // (CI/секреты имеют приоритет над .env)
            if std::env::var(key).is_ok() {
                println!("cargo:warning=[env] {} уже в окружении, пропускаю", key);
                continue;
            }

            // Прокидываем в компилятор: option_env!("KEY") увидит значение
            println!("cargo:rustc-env={}={}", key, value);
            // Диагностика: увидим в консоли сборки, что ключ подхвачен
            println!(
                "cargo:warning=[env] {} = {}",
                key,
                if value.is_empty() { "<ПУСТО!>" } else { "<set>" }
            );

            loaded_any = true;
        }

        // Пересобирать при изменении .env
        println!("cargo:rerun-if-changed={}", path.display());
    }

    if !loaded_any {
        println!("cargo:warning=[env] .env не найден или пуст. Ищи в: {}", project_root.display());
    }

    // На всякий случай — пересборка при изменении самого build.rs
    println!("cargo:rerun-if-changed=build.rs");
}