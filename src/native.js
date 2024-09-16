//Finality Framework Native Abstract Layer
//This is Tauri(Desktop) backend.
//You should add new functions in this js file if you want to add a feature in native world.
//When porting Finality to other platform,reimplementing these functions is required.
const { relaunch, exit } = window.__TAURI__.process
const { invoke } = window.__TAURI__.tauri
const { sep } = window.__TAURI__.path
const { appWindow } = window.__TAURI__.window;
const { open, message } = window.__TAURI__.dialog

export async function jre_exists() {
    return await invoke("jre_exists");
}
export async function exit_program() {
    await exit();
}

function get_better_path(path_str) {
    if (path_str.substr(-1) == get_sep()) {
        return path_str
    } else {
        return path_str + get_sep();
    }
}

export async function open_file_chooser(isDirectory) {
    let path = await open({ directory: isDirectory })
    if(isDirectory){
        return get_better_path(path)
    }else{
        return path
    }
}

export async function open_website(url) {
    await invoke("open_website", { url: url })
}

export async function set_title(title) {
    appWindow.setTitle(title);
}

export async function reload_config() {
    await invoke("reload_config")
}

export async function switch_mod(id, enabled) {
    await invoke("switch_mod", { id: id, enabled: enabled })
}

export async function extract_mod_zip(path){
    await invoke("extract_mod_zip", { zipPath: path })
}

export async function setDecoration(decoration) {
    appWindow.setDecorations(decoration);
}

export async function reboot() {
    await relaunch();
}

export function get_sep() {
    return sep;
}

export async function get_mods() {
    return await invoke("get_mods");
}

export async function load_mods() {
    await invoke("load_mods");
}

export async function get_platform() {
    return "DESKTOP";
}

export async function get_text(key) {
    return await invoke("get_text", { key: key })
}

export async function is_a_vaild_game_path(path) {
    return await invoke("is_a_vaild_game_path", { path: path });
}
export async function get_language() {
    return await invoke("get_language");
}
export async function set_lang(lang_name) {
    await invoke("set_lang", { langName: lang_name })
}
export async function save_config() {
    await invoke("save_config")
}

export async function extract_bootstrap() {
    await invoke("extract_bootstrap")
}

export async function launch_game() {
    await invoke("launch_game")
}

export async function is_oobe_over() {
    return await invoke("is_oobe_over")
}

export async function set_oobe_over(oobe_over) {
    await invoke("set_oobe_over", { oobeOver2: oobe_over })
}