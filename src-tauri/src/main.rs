#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::io;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use winreg::RegKey;

const PREFIX: &str = r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall";

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
  let uninstall = RegKey::predef(HKEY_LOCAL_MACHINE)
    .open_subkey_with_flags(PREFIX, KEY_READ)
    .unwrap();

  let keys = &uninstall.enum_keys().collect::<Vec<io::Result<String>>>();

  vec![
    "Google Chrome",
    "Google Chrome Dev",
    "Google Chrome Beta",
    "Google Chrome Canary",
    "Microsoft Edge",
    "Microsoft Edge Dev",
    "Microsoft Edge Beta",
    "Microsoft Edge Canary",
    "Microsoft Edge Dev Preview",
    "Microsoft Edge Preview",
  ]
  .iter()
  .map(|browser| Browser {
    name: browser
      .trim_start_matches("Google ")
      .trim_start_matches("Microsoft ")
      .to_string(),
    installed: keys.iter().any(|it| it.as_ref().unwrap() == browser),
  })
  .collect::<Vec<Browser>>()
}

#[tauri::command]
fn passwordslist(_browser: String) -> Vec<Password> {
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
