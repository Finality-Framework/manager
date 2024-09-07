const { open,message } = window.__TAURI__.dialog
import * as native from "./native.js"
let selectedPathEl
let nextBtnEl

function get_better_path(path_str){
  if(path_str.substr(-1) == native.get_sep()){
    return path_str
  }else{
    return path_str+native.get_sep();
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
    message(await native.get_text("oobe.invaild_game_path_please_check"),{title:"Finality Framework",type:"error"})
    selectedPathEl.value = ""
  }else{
    await native.set_oobe_over(true)
    await native.save_config()
    await native.extract_bootstrap()
    window.location.href = "./main.html"
  }
}
window.addEventListener("DOMContentLoaded", async () => {
  if(await native.is_oobe_over()){
    window.location.href = "./main.html"
  }
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
