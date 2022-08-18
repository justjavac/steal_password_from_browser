#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use serde::Serialize;
use std::env;
use std::io;

#[cfg(windows)]
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
const PREFIX: &str = r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall";

const BROWSERS: [&str; 10] = [
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
];

#[derive(Serialize)]
struct Browser {
  name: String,
  installed: bool,
}

#[derive(Serialize)]
struct Password {
  url: String,
  username: String,
  password: String,
}

#[tauri::command]
fn browserslist() -> Vec<Browser> {
  #[cfg(windows)]
  let keys = RegKey::predef(HKEY_LOCAL_MACHINE)
    .open_subkey_with_flags(PREFIX, KEY_READ)
    .unwrap()
    .enum_keys()
    .collect::<Vec<io::Result<String>>>();

  #[cfg(not(windows))]
  let keys: Vec<io::Result<String>> = vec![
    io::Result::Ok("Google Chrome".to_string()),
    io::Result::Ok("Google Chrome Canary".to_string()),
  ];

  BROWSERS
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
#[cfg(not(windows))]
#[allow(unused_variables)]
fn passwordslist(browser: String) -> Vec<Vec<String>> {
  vec![
    vec![
      "https://www.github.com".to_string(),
      "admin".to_string(),
      "12345678".to_string(),
    ],
    vec![
      "https://www.baidu.com".to_string(),
      "root".to_string(),
      "12345678".to_string(),
    ],
    vec![
      "https://www.google.com".to_string(),
      "justjavac".to_string(),
      "12345678".to_string(),
    ],
  ]
}

#[tauri::command]
#[cfg(windows)]
fn passwordslist(browser: String) -> Vec<Vec<String>> {
  use chrome_password::{get_master_key, get_password};
  use std::path::PathBuf;

  let user_profile = env::var("LOCALAPPDATA").unwrap();
  let browser_path = PathBuf::from(&user_profile)
    .join(if BROWSERS.iter().position(|it| it.contains(&browser)).unwrap() > 3 {
      "Microsoft"
    } else {
      "Google"
    })
    .join(&browser);
  let local_state_path = browser_path.join("User Data/Local State");
  let login_data_path = browser_path.join("User Data/Default/Login Data");

  get_password(&login_data_path, &get_master_key(&local_state_path))
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![browserslist, passwordslist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
