//mod is a keyword of rust.
use serde_json::{self, Value};
use std::{fs::{self, File}, path::Path};
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
    pub fn load(path: &str) -> ModInstance {
        let mut disabled = false;
        let disabled_flag_file_path = "".to_string()+path+"DISABLED";
        if Path::new(&disabled_flag_file_path).exists() {
            disabled = true;
        }
        let path2 = path.to_string();
        let mod_txt = fs::read_to_string(path.to_string() + "mod.txt").unwrap();
        let mod_txt: Value = serde_json::from_str(&mod_txt).unwrap();
        ModInstance {
            path: path2,
            enabled: !disabled,
            name: mod_txt["Name"].as_str().unwrap().to_string(),
            description: mod_txt["Description"].as_str().unwrap().to_string(),
            tags: ["1".to_string(), "2".to_string()].to_vec(),
            change_note: mod_txt["ChangeNote"].as_str().unwrap().to_string(),
            id: 0,
        }
    }
    pub fn enable(&mut self) {
        let disabled_flag_file_path = "".to_string()+&self.path+"DISABLED";
        if !&self.enabled {
            if Path::new(&disabled_flag_file_path).exists() {
                fs::remove_file(Path::new(&disabled_flag_file_path)).unwrap();
            }
            self.enabled = true;
        }
    }
    pub fn disable(&mut self) {
        let disabled_flag_file_path = "".to_string()+&self.path+"DISABLED";
        if self.enabled {
            if !Path::new(&disabled_flag_file_path).exists() {
                File::create(Path::new(&disabled_flag_file_path)).unwrap();
            }
            self.enabled = false;
        }
    }
}
