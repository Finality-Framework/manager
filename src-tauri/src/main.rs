// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::env_manager::Env;
use config_manager::Config;
use env_manager::ENV;
use language_manager::load_locale_text;
use tauri_commands::{extract_bootstrap, get_language, get_text, is_a_vaild_game_path, is_oobe_over, launch_game, load_mod, reload_config, save_config, set_lang, set_oobe_over};

mod config_manager;
mod consts;
mod env_manager;
mod language_manager;
mod mod_manager;
pub mod tauri_commands;



fn main() {
    unsafe {
        ENV = Env {
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
            load_mod,
            is_a_vaild_game_path,
            get_text,
            reload_config,
            set_lang,
            get_language,
            save_config,
            extract_bootstrap,
            launch_game,
            is_oobe_over,
            set_oobe_over
        ])
        .run(tauri::generate_context!())
        .expect("error while running Finality Framework");
}
