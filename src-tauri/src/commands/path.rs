use std::ops::Add;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Paths {
    pub(crate) app_data: String,
    pub(crate) cache_dir: String,
    pub(crate) config_dir: String,
}

lazy_static! {
    pub static ref GLOBAL_PATHS: Arc<Mutex<Paths>> = Arc::new(Mutex::new(Paths {
        app_data: String::new(),
        cache_dir: String::new(),
        config_dir: String::new(),
    }));
}

#[tauri::command]
pub async fn save_paths(appData: String, cacheDir: String, configDir: String) {
    {
        let mut paths = GLOBAL_PATHS.lock().expect("Failed to lock GLOBAL_PATHS");
        paths.app_data = appData;
        paths.cache_dir = cacheDir.add("\\fakeLibrus\\");
        paths.config_dir = configDir.add("\\fakeLibrus\\");
    }
    
    if let Err(e) = crate::utils::database::init_db() {
        eprintln!("Error initializing database: {:?}", e);
    }
}
