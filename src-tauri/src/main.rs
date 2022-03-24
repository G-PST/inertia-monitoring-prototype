#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri_plugin_fs_watch::Watcher;

#[derive(Serialize, Deserialize, Debug)]
struct Generator {
  name: String,
  bus: i64,
  pmin: f64,
  pmax: f64,
  basemva: f64,
  droop: f64,
  deadband: f64,
  h: f64,
}

#[tauri::command]
fn get_system_data(data: String) -> Result<Vec<Generator>, String> {
  let p = PathBuf::from(data);
  let mut generators = vec![];
  match csv::Reader::from_path(p) {
    Ok(mut rdr) => {
      for result in rdr.deserialize() {
        match result {
          Ok(record) => generators.push(record),
          Err(e) => return Err(e.to_string()),
        }
      }
      Ok(generators)
    },
    Err(e) => Err(e.to_string()),
  }
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::WindowState::default())
    .plugin(Watcher::default())
    .invoke_handler(tauri::generate_handler![get_system_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
