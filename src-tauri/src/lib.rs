pub mod utils;
pub mod commands;
mod test;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::path::save_paths,
            commands::database::get_all,
            commands::database::add_new,
            commands::database::remove_teacher,
            commands::database::import_csv,
            commands::database::export_csv,
            commands::database::update_teacher
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
