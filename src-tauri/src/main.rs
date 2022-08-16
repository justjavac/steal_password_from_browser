#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn browser_list() -> Vec<i8> {
  vec![1, 2, 3]
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, browser_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
