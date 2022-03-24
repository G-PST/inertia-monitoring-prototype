#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::path::PathBuf;

#[tauri::command]
async fn update_system_data(data: String) -> Result<String, String> {
  println!("I was invoked from JS, with this message: {}", data);
  let p = PathBuf::from(data);
  dbg!(p);
  Ok("Hello from Rust!".into())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![update_system_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
