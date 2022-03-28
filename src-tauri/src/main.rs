#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri_plugin_fs_watch::Watcher;

#[derive(Serialize, Deserialize, Debug)]
struct Generator {
  name: String,
  kind: String,
  bus: i64,
  mva: f64,
  pmin: f64,
  pmax: f64,
  basemva: f64,
  droop: f64,
  deadband: f64,
  h: f64,
}

#[tauri::command]
fn get_commitment_data(data: String) -> Result<Vec<HashMap<String, String>>, String> {
  let p = PathBuf::from(data);
  let mut commitments = vec![];
  match csv::Reader::from_path(p) {
    Ok(mut rdr) => {
      let headers = if let Ok(headers) = rdr.headers() {
        headers.clone()
      } else {
        return Err("Unable to parse CSV Headers".to_string());
      };
      for result in rdr.records() {
        if let Ok(record) = result {
          let mut commitment: HashMap<String, String> = Default::default();
          let records: Vec<String> = record.iter().map(|s| s.to_string()).collect();
          for (h, r) in headers.iter().zip(records) {
            commitment.insert(h.to_string(), r);
          }
          commitments.push(commitment);
        } else {
          return Err("Unable to parse CSV Row".to_string());
        }
      }
      Ok(commitments)
    },
    Err(e) => Err(e.to_string()),
  }
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
    .invoke_handler(tauri::generate_handler![get_system_data, get_commitment_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
