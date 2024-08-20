use std::fs::{self};
use serde_json::Value;

pub fn load_locale_text(path:&str) -> Vec<Value>{
    let locale_json = fs::read_to_string(path);
    if let Ok(content) = locale_json{
        println!("load locale ok");
        let values:Value = serde_json::from_str(&content).unwrap();
        let mut locale_vec = Vec::new();
        locale_vec.push(values);
        return locale_vec
    }else {
        return Vec::new();
    }
}