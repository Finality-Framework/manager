// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config_manager::Config;
use language_manager::load_locale_text;
use mod_manager::ModInstance;
use serde_json::Value;
use std::{fs::{self, File}, sync::Mutex};

pub mod config_manager;
mod language_manager;
mod mod_manager;
mod consts;

pub static mut ENV: Env = Env {
    sync_lock: Mutex::new(0),
    config: Vec::new(),
    mod_list: Vec::new(),
    locale_text_map: Vec::new(),
};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_lang(app: tauri::AppHandle, lang_name: &str) {
    unsafe {
        let _unused = ENV.sync_lock.lock().unwrap();
        let config = ENV.config.pop();
        if let Some(mut config2) = config {
            println!("{}", lang_name);
            config2.lang = lang_name.to_string();
            let resource_path = app
                .path_resolver()
                .resolve_resource(consts::LANG_PATH_PREFIX.to_string() + lang_name + ".json")
                .expect("failed to resolve resource");
            ENV.locale_text_map =
                load_locale_text(&resource_path.into_os_string().into_string().unwrap());
            println!("2 {}", &config2.lang);
            ENV.config.push(config2);
        }
    }
}

#[tauri::command]
fn extract_bootstrap(app: tauri::AppHandle) {
    let resource_path = app
        .path_resolver()
        .resolve_resource(consts::BOOTSTRAP_PATH.to_string())
        .expect("failed to resolve resource");
    let mut game_path = "".to_string();
    unsafe {
        let config = ENV.config.pop().unwrap();
        game_path = config.game_path.to_string();
    }
    let _ = fs::copy(&resource_path.into_os_string().into_string().unwrap(),game_path );
}

#[tauri::command]
fn reload_config(app: tauri::AppHandle) {
    unsafe {
        let _unused = ENV.sync_lock.lock();
        let config = ENV.config.pop();
        if let Some(mut config2) = config {
            println!("reload config...");
            config2 = Config::get_config_from_json("./config.json");
            ENV.config.push(config2);
            let resource_path = app
                .path_resolver()
                .resolve_resource("languages/".to_string() + &get_language() + ".json")
                .expect("failed to resolve resource");
            ENV.locale_text_map =
                load_locale_text(&resource_path.into_os_string().into_string().unwrap())
        }
    }
}

#[tauri::command]
fn is_a_vaild_game_path(path: &str) -> bool {
    if let Ok(_file) = File::open(path.to_string() + "AoC2.exe") {
        unsafe {
            let mut config = ENV.config.pop().unwrap();
            config.game_path = path.to_string();
            ENV.config.push(config);
        }
        println!("OK");
        true
    } else {
        println!("NO");
        false
    }
}

#[tauri::command]
fn load_mod(name: &str) -> String {
    return mod_manager::ModInstance::load(&("".to_string() + name + "/")).description;
}
#[tauri::command]
fn get_text(key: &str) -> String {
    let mut locale_text = "NO TEXT".to_string();
    unsafe {
        let values = ENV.locale_text_map.pop().unwrap();
        if let Some(locale_text_2) = values[key].as_str() {
            locale_text = locale_text_2.to_string();
        }
        ENV.locale_text_map.push(values);
    }
    locale_text
}

#[tauri::command]
fn get_language() -> String {
    let mut lang_name = "zh_cn".to_string();
    unsafe {
        let config = ENV.config.pop();
        if let Some(config2) = config {
            lang_name = (&config2.lang).to_string();
            ENV.config.push(config2);
        }
    }
    lang_name
}

#[tauri::command]
fn save_config() {
    unsafe {
        ENV.config.get(0).unwrap().save_config("./config.json");
    }
}

fn main() {
    unsafe {
        ENV = Env {
            sync_lock: Mutex::new(0),
            config: vec![Config::get_config_from_json("./config.json")],
            mod_list: Vec::new(),
            locale_text_map: Vec::new(),
        };
    }
    tauri::Builder::default()
        .setup(|app| {
            let resource_path = app
                .path_resolver()
                .resolve_resource("resources/lang/".to_string() + &get_language() + ".json")
                .expect("failed to resolve resource");
            unsafe {
                ENV.locale_text_map =
                    load_locale_text(&resource_path.into_os_string().into_string().unwrap())
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            load_mod,
            is_a_vaild_game_path,
            get_text,
            reload_config,
            set_lang,
            get_language,
            save_config,
            extract_bootstrap
        ])
        .run(tauri::generate_context!())
        .expect("error while running Finality Framework");
}

pub struct Env {
    //static变量强制提前声明但又不让使用non-const func到底是哪个天才想出来的主意......
    //先这样了 有好办法发issues踹我一脚
    pub sync_lock: Mutex<i32>,
    pub config: Vec<Config>,
    pub mod_list: Vec<ModInstance>,
    pub locale_text_map: Vec<Value>,
}
