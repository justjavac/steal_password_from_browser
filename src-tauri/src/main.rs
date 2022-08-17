#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Browser {
  name: String,
  installed: bool,
}

#[derive(Serialize, Deserialize)]
struct Password {
  url: String,
  username: String,
  password: String,
}

#[tauri::command]
fn browserslist() -> Vec<Browser> {
  vec![
    Browser {
      name: "Chrome".to_string(),
      installed: true,
    },
    Browser {
      name: "Chrome Dev".to_string(),
      installed: true,
    },
    Browser {
      name: "Chrome Beta".to_string(),
      installed: false,
    },
    Browser {
      name: "Chrome Canary".to_string(),
      installed: true,
    },
    Browser {
      name: "Edge".to_string(),
      installed: true,
    },
    Browser {
      name: "Edge Dev".to_string(),
      installed: false,
    },
    Browser {
      name: "Edge Beta".to_string(),
      installed: false,
    },
    Browser {
      name: "Edge Canary".to_string(),
      installed: true,
    },
    Browser {
      name: "Edge Dev Preview".to_string(),
      installed: false,
    },
    Browser {
      name: "Edge Preview".to_string(),
      installed: false,
    },
  ]
}

#[tauri::command]
fn passwordslist(browser: String) -> Vec<Password> {
  println!("{}", browser);
  vec![
    Password {
      url: "https://github.com".to_string(),
      username: "admin".to_string(),
      password: "12345678".to_string(),
    },
    Password {
      url: "https://baidu.com".to_string(),
      username: "root".to_string(),
      password: "12345678".to_string(),
    },
    Password {
      url: "https://google.com".to_string(),
      username: "justjavac".to_string(),
      password: "12345678".to_string(),
    },
  ]
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![browserslist, passwordslist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
