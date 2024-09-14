//mod is a keyword of rust.
use crate::env_manager::{self, ENV};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{
    fs::{self, File},
    path::Path,
};

pub fn load_mods() {
    unsafe {
        let mut mod_list: Vec<ModInstance> = Vec::new();
        let config = ENV.config.pop().unwrap();
        let game_path = config.game_path.to_string();
        ENV.config.push(config);
        let mod_path = game_path + "mod/";
        let mod_path2 = Path::new(&mod_path);
        if !mod_path2.exists() {
            fs::create_dir(mod_path2).unwrap();
        }
        for folder in fs::read_dir(mod_path).unwrap() {
            let dir_entry = folder.unwrap();
            if let Some(instance) = ModInstance::load(dir_entry.path().to_str().unwrap()){
                mod_list.push(instance);
            }
        }
        ENV.mod_list = mod_list;
    }
}

pub fn build_manifest() {
    unsafe {
        println!("Building manifest");
        println!("modlist size {}",&ENV.mod_list.len().to_string());
        let config = ENV.config.pop().unwrap();
        let game_path = config.game_path.to_string();
        let mut manifest_str: String = String::from("");
        ENV.config.push(config);
        for mod_instance in &ENV.mod_list {
            if mod_instance.enabled {
                manifest_str.push_str(&mod_instance.path);
                manifest_str.push_str(";");
            }
        }
        let _ = fs::write(game_path + "manifest.txt", manifest_str);
    }
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModInstance {
    pub path: String,
    pub enabled: bool,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub change_note: String,
    pub id: u64,
}

impl ModInstance {
    pub fn load(path: &str) -> Option<ModInstance> {
        let mut disabled = false;
        let disabled_flag_file_path = "".to_string() + path + "/DISABLED";
        if Path::new(&disabled_flag_file_path).exists() {
            disabled = true;
        }
        let path2 = path.to_string();
        let mod_txt = fs::read_to_string(path.to_string() + "/mod.txt");
        if let Ok(_) = mod_txt{

        }else{
            println!("{}", path.to_string() + "mod.txt");
            println!("None modtxt");
            return None;
        }
        let mod_txt = mod_txt.unwrap();
        let id = fs::read_to_string(path.to_string() + "/id.txt");
        if let Ok(id) = id {
            let id = id.parse::<u64>().unwrap();
            let mod_txt: Value = json5::from_str(&mod_txt).unwrap();
            Some(ModInstance {
                path: path2,
                enabled: !disabled,
                name: mod_txt["Name"].as_str().unwrap().to_string(),
                description: mod_txt["Description"].as_str().unwrap().to_string(),
                tags: ["1".to_string(), "2".to_string()].to_vec(),
                change_note: mod_txt["ChangeNote"].as_str().unwrap().to_string(),
                id: id,
            })
        } else {
            println!("None id");
            return None;
        }
    }
    pub fn enable(&mut self) {
        let disabled_flag_file_path = "".to_string() + &self.path + "/DISABLED";
        if !&self.enabled {
            if Path::new(&disabled_flag_file_path).exists() {
                fs::remove_file(Path::new(&disabled_flag_file_path)).unwrap();
            }
            self.enabled = true;
        }
    }
    pub fn disable(&mut self) {
        let disabled_flag_file_path = "".to_string() + &self.path + "/DISABLED";
        if self.enabled {
            if !Path::new(&disabled_flag_file_path).exists() {
                File::create(Path::new(&disabled_flag_file_path)).unwrap();
            }
            self.enabled = false;
        }
    }
}
