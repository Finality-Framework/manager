use std::{
    fs::{self, File},
    process::Command,
};

use crate::{
    config_manager::{self, Config},
    consts,
    env_manager::ENV,
    language_manager::load_locale_text,
    mod_manager,
};

#[tauri::command]
pub fn is_oobe_over() -> bool {
    let is_oobe_over;
    unsafe {
        let config = ENV.config.pop().unwrap();
        is_oobe_over = (&config.oobe_over).to_owned();
        ENV.config.push(config);
    }
    return is_oobe_over;
}

#[tauri::command]
pub fn set_oobe_over(oobe_over2: bool) {
    unsafe {
        let mut config = ENV.config.pop().unwrap();
        config.oobe_over = oobe_over2;
        ENV.config.push(config);
    }
}
#[tauri::command]
pub fn load_mod(name: &str) {
    unsafe {
        ENV.mod_list.push(mod_manager::ModInstance::load(&("".to_string() + name + "/")));
    }
}
#[tauri::command]
pub fn get_text(key: &str) -> String {
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
pub fn set_lang(app: tauri::AppHandle, lang_name: &str) {
    unsafe {
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
pub fn extract_bootstrap(app: tauri::AppHandle) {
    println!("extract bootstrap");
    let resource_path = app
        .path_resolver()
        .resolve_resource(consts::BOOTSTRAP_PATH.to_string())
        .expect("failed to resolve resource");
    let game_path: String;
    unsafe {
        let config = ENV.config.pop().expect("Config struct not found!");
        game_path = config.game_path.to_string();
        ENV.config.push(config);
    }
    let _ = fs::copy(
        &resource_path.into_os_string().into_string().unwrap(),
        game_path + "bootstrap.jar",
    );
}

#[tauri::command]
pub fn reload_config(app: tauri::AppHandle) {
    unsafe {
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
pub fn is_a_vaild_game_path(path: &str) -> bool {
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
pub fn get_language() -> String {
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
pub fn save_config() {
    unsafe {
        ENV.config.get(0).unwrap().save_config("./config.json");
    }
}

#[tauri::command]
pub fn launch_game() {
    let game_path: String;
    unsafe {
        let config = ENV.config.pop().unwrap();
        game_path = config.game_path.to_string();
        ENV.config.push(config);
    }
    //launch game!
    let _child = Command::new("javaw")
        //文二实测不加utf-8加载某些mod的时候会出现乱码
        .arg("-Dfile.encoding=utf-8")
        .arg("-jar")
        .arg((&game_path).to_string() + "bootstrap.jar")
        .arg((&game_path).to_string() + "manifest.txt")
        .current_dir(&game_path)
        .spawn()
        .unwrap();
}
