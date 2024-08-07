use std::{fs::{self, OpenOptions}, io::Write};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize,Deserialize)]
pub struct Config{
    
    pub game_path:String,
    pub lang:String
}

impl Config{
    pub fn get_config_from_json(path:&str) -> Config{
        let config_json = fs::read_to_string(path);
        if let Ok(content) = config_json{
            let values:Value = serde_json::from_str(&content).unwrap();
            Config{
                game_path:values["game_path"].as_str().unwrap().to_string(),
                lang : values["lang"].as_str().unwrap().to_string()
            }
        }else {
            println!("Failed to open config file,use default config!");
            build_default_config().save_config(path);
            build_default_config()
        }
    }
    pub fn save_config(&self,path:&str){
        println!("saving config...");
        let mut file = OpenOptions::new().read(true).write(true).create(true).truncate(true).open(path).unwrap();
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes()).unwrap();
    }
}

pub fn build_default_config() -> Config{
    Config{
        game_path:"".to_string(),
        lang:"en_us".to_string()
    }
}