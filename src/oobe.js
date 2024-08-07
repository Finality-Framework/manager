const { invoke } = window.__TAURI__.tauri
const { open,message } = window.__TAURI__.dialog
import * as native from "./native.js"
import * as localize from "./localize.js"
let selectedPathEl
let selected
let nextBtnEl

function get_better_path(path_str){
  if(path_str.substr(-1) == "/"){
    return path_str
  }else{
    return path_str+"/";
  }
}


async function greet() {
  let path = await open({directory:true})
  path = get_better_path(path);
  if(!await native.is_a_vaild_game_path(path)){
    message(await native.get_text("oobe.invaild_game_path"),{title:"Finality Framework",type:"error"})
    //selectedPathEl.value = "未选择游戏路径"
  }else{
    selectedPathEl.value = path
  }
}

async function confirm_next(){
  selectedPathEl.value = get_better_path(selectedPathEl.value);
  if(!await native.is_a_vaild_game_path(selectedPathEl.value)){
    message("游戏实例路径无效，请检查后重新填入。",{title:"Finality Framework",type:"error"})
    selectedPathEl.value = ""
  }else{
    await native.save_config()
    window.location.href = "./main.html"
  }
}
window.addEventListener("DOMContentLoaded", () => {
  let chooseLangEl = document.querySelector("#choose_lang")
  native.get_language().then((value)=>{chooseLangEl.value = value})
  chooseLangEl.addEventListener("change",async function(event){
    await native.set_lang(event.target.value);
    location.reload();
  })
  nextBtnEl = document.querySelector("#nextBtn");
  nextBtnEl.onclick = function(){confirm_next()}
  selectedPathEl = document.querySelector("#selectedPath");
  selectedPathEl.addEventListener("keydown",function(event){
    if(event.keyCode == 13){
      confirm_next();
      event.preventDefault();
    }
  });
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
