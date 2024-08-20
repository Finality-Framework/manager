import * as native from "./native.js";

window.addEventListener("DOMContentLoaded", () => {
    let nextBtnEl = document.querySelector("#launch");
    nextBtnEl.onclick = function(){
        native.launch_game();
    }
})