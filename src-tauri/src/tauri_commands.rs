use std::{
    fs::{self, File}, io::{self, Write}, process::Command
};

use crate::{
    config_manager::Config,
    consts,
    env_manager::ENV,
    language_manager::load_locale_text,
    mod_manager::{self, ModInstance},
};


#[tauri::command]
pub fn remove_mod(id:u64){
    mod_manager::remove_mod(id);
}

#[tauri::command]
pub fn extract_mod_zip(zip_path:&str){
    println!("extract_mod_zip: {}",zip_path);
    if zip_path.is_empty() || !zip_path.ends_with(".zip") {
        println!("zip_path is empty or not end with .zip");
        return;
    }
    let name = zip_path.split(std::path::MAIN_SEPARATOR_STR).last().unwrap().replace(".zip", "");
    let mut zip = zip::ZipArchive::new(File::open(zip_path).unwrap()).unwrap();
    let mut mod_txt_exists = false;
    let mut id_txt_exists = false;
    //判断zip内是否存在mod.txt文件和id.txt文件
    for i in 0..zip.len() {
        println!("file name: {}", zip.by_index(i).unwrap().name());
        let file = zip.by_index(i).unwrap();
        if file.name() == "mod.txt" {  
            mod_txt_exists = true;
        }
        if file.name() == "id.txt" {
            id_txt_exists = true;
        }
    }
    if mod_txt_exists && id_txt_exists {
        let mut game_path = String::new();
        //解压文件
        unsafe{
            let config = ENV.config.pop().unwrap();
            game_path = config.game_path.to_string();
            ENV.config.push(config);
        }
        println!("extract at {}", (&game_path).to_string()+"mod"+std::path::MAIN_SEPARATOR_STR+&name);
        zip.extract(game_path+"mod"+std::path::MAIN_SEPARATOR_STR+&name).unwrap();
    }
}

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
pub fn load_mods() {
    println!("load mods");
    mod_manager::load_mods();
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
pub async fn reload_config(app: tauri::AppHandle) {
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
pub fn get_mods() -> Vec<ModInstance> {
    unsafe {
        let mut mod_list: Vec<ModInstance> = Vec::new();
        for instance in &ENV.mod_list {
            let instance = instance.clone();
            mod_list.push(instance);
        }
        return mod_list;
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
pub fn open_website(url: &str) {
    let _ = Command::new("cmd").arg("/C").arg("start").arg(url).spawn();
}

#[tauri::command]
pub fn jre_exists() -> bool {
    //通过java -version命令判断jre是否存在
    println!("jre_exists");
    let output = Command::new("java").arg("-Dfile.encoding=utf-8").arg("-version").output();
    if let Ok(output) = output {
        io::stdout().write_all(&output.stdout).unwrap();
        println!("2");
        if output.status.success() {
            println!("3");
            return true;
        } else {
            return false;
        }
    }
    return false;
}
#[tauri::command]
pub async fn switch_mod(id: u64, enabled: bool) {
    println!("switch mod id:{} enabled:{}", id, enabled);
    unsafe {
        let mut mod_list = ENV.mod_list.clone();
        for instance in &mut mod_list {
            if instance.id == id {
                instance.switch(enabled)
            }
        }
    }
}

#[tauri::command]
pub fn launch_game() {
    let game_path: String;

    mod_manager::load_mods();
    mod_manager::build_manifest();
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
