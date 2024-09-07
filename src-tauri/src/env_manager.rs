use crate::mod_manager::ModInstance;
use serde_json::Value;
use crate::config_manager::Config;

pub static mut ENV: Env = Env {
    config: Vec::new(),
    mod_list: Vec::new(),
    locale_text_map: Vec::new(),
};

pub struct Env {
    //static变量强制提前声明但又不让使用non-const func到底是哪个天才想出来的主意......
    //先这样了 有好办法发issues踹我一脚
    pub config: Vec<Config>,
    pub mod_list: Vec<ModInstance>,
    pub locale_text_map: Vec<Value>,
}

impl Env {
    
}
