//! Android WiFi MulticastLock для mDNS.
//!
//! Android по умолчанию фильтрует multicast-пакеты для экономии батареи.
//! Без этого лока `mdns-sd` запускается, шлёт запросы, но НИКОГДА не получает ответы.
//! Компилируется только на Android (`#[cfg(target_os = "android")]`).

#[cfg(target_os = "android")]
mod inner {
    use jni::objects::{GlobalRef, JObject, JValue};
       use jni::JavaVM;
    use std::sync::Mutex;

    static LOCK: Mutex<Option<GlobalRef>> = Mutex::new(None);

    pub fn acquire() -> Result<(), String> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }
            .map_err(|e| format!("JavaVM::from_raw: {}", e))?;
        let context = unsafe { JObject::from_raw(ctx.context().cast()) };

        let mut env = vm
            .attach_current_thread()
            .map_err(|e| format!("attach_current_thread: {}", e))?;

        // WifiManager wifi = (WifiManager) context.getSystemService(Context.WIFI_SERVICE);
        let wifi_service = env
            .get_static_field(
                "android/content/Context",
                "WIFI_SERVICE",
                "Ljava/lang/String;",
            )
            .map_err(|e| format!("get WIFI_SERVICE: {}", e))?;
        let wifi_service_obj = wifi_service
            .l()
            .map_err(|e| format!("WIFI_SERVICE to object: {}", e))?;

        let wifi_manager = env
            .call_method(
                &context,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[JValue::Object(&wifi_service_obj)],
            )
            .map_err(|e| format!("getSystemService: {}", e))?
            .l()
            .map_err(|e| format!("getSystemService to object: {}", e))?;

        if wifi_manager.is_null() {
            return Err("WifiManager is null".to_string());
        }

        // MulticastLock lock = wifi.createMulticastLock("35mmclub-mdns");
        let tag = env
            .new_string("35mmclub-mdns")
            .map_err(|e| format!("new_string: {}", e))?;
        let lock = env
            .call_method(
                &wifi_manager,
                "createMulticastLock",
                "(Ljava/lang/String;)Landroid/net/wifi/WifiManager$MulticastLock;",
                &[JValue::Object(&tag)],
            )
            .map_err(|e| format!("createMulticastLock: {}", e))?
            .l()
            .map_err(|e| format!("createMulticastLock to object: {}", e))?;

        // lock.setReferenceCounted(false);
        env.call_method(
            &lock,
            "setReferenceCounted",
            "(Z)V",
            &[JValue::Bool(0)],
        )
        .map_err(|e| format!("setReferenceCounted: {}", e))?;

        // lock.acquire();
        env.call_method(&lock, "acquire", "()V", &[])
            .map_err(|e| format!("acquire: {}", e))?;

        // Сохраняем глобальную ссылку, чтобы GC не убил лок
        let global = env
            .new_global_ref(&lock)
            .map_err(|e| format!("new_global_ref: {}", e))?;

        let mut guard = LOCK.lock().unwrap();
        *guard = Some(global);

        println!("[sync] MulticastLock acquired");
        Ok(())
    }

    pub fn release() {
        let mut guard = LOCK.lock().unwrap();
        if let Some(global) = guard.take() {
            let ctx = ndk_context::android_context();
            if let Ok(vm) = unsafe { JavaVM::from_raw(ctx.vm().cast()) } {
                if let Ok(mut env) = vm.attach_current_thread() {
                    let lock = global.as_obj();
                    let _ = env.call_method(lock, "release", "()V", &[]);
                }
            }
            println!("[sync] MulticastLock released");
        }
    }
}

#[cfg(not(target_os = "android"))]
mod inner {
    pub fn acquire() -> Result<(), String> {
        Ok(()) // no-op на десктопе
    }
    pub fn release() {}
}

pub use inner::{acquire, release};