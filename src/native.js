const { invoke } = window.__TAURI__.tauri
const { sep } = window.__TAURI__.path
const {appWindow} = window.__TAURI__.window;
export async function reload_config(){
    await invoke("reload_config")
}

export async function setDecoration(decoration){
    appWindow.setDecorations(decoration);
}

export function get_sep(){
    return sep;
}

export async function get_mods(){
    return await invoke("get_mods");
}

export async function load_mods(){
    await invoke("load_mods");
}

export async function get_platform(){
    return "DESKTOP";
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
    await invoke("set_lang",{langName:lang_name})
}
export async function save_config(){
    await invoke("save_config")
}

export async function extract_bootstrap(){
    await invoke("extract_bootstrap")
}

export async function launch_game() {
    await invoke("launch_game")
}

export async function is_oobe_over() {
    return await invoke("is_oobe_over")
}

export async function set_oobe_over(oobe_over){
    await invoke("set_oobe_over",{oobeOver2:oobe_over})
}