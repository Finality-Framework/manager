const { invoke } = window.__TAURI__.tauri

export async function reload_config(){
    await invoke("reload_config")
}
  
export async function get_text(key){
    return await invoke("get_text",{key : key})
}

export async function is_a_vaild_game_path(path){
    return await invoke("is_a_vaild_game_path",{ path : path});
}
export async function get_language(){
    return await invoke("get_language");
}
export async function set_lang(lang_name){
    console.log("1")
    await invoke("set_lang",{langName:lang_name})
    console.log("2")
}
export async function save_config(){
    await invoke("save_config")
}
