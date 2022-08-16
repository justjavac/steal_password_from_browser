#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

#[tauri::command]
fn browserslist() -> Vec<i8> {
  vec![1, 2, 3]
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![browserslist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
