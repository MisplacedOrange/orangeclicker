// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn start_clicking() {
    // TODO: start click loop/task
}

#[tauri::command]
fn stop_clicking() {
    // TODO: stop click loop/task
}

#[tauri::command]
fn set_clicks_per_second(cps: u32) {
    // TODO: update click rate while running or before start
    let _ = cps;
}

#[tauri::command]
fn foreground_clicks(enabled: bool) {
    // TODO: enforce foreground-only clicks
    let _ = enabled;
}

#[tauri::command]
fn taskbar_clicks(enabled: bool) {
    // TODO: allow/disallow clicking in taskbar
    let _ = enabled;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_clicking,
            stop_clicking,
            set_clicks_per_second,
            target_foreground,
            taskbar_clicks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
