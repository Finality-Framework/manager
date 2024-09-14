import * as native from "./native.js";
import * as nav from "./nav.js"
window.addEventListener("DOMContentLoaded", () => {
    native.setDecoration(true);
    let modCardTemplate = document.querySelector("#mod-card-template")
    class ModCard extends HTMLElement {
        constructor() {
            super()
            let shadow = this.attachShadow({ mode: 'open' })
            shadow.appendChild(modCardTemplate.content.cloneNode(true))
        }
    }
    window.customElements.define('mod-card', ModCard)
    let nextBtnEl = document.querySelector("#launch");
    nextBtnEl.onclick = function () {
        native.launch_game();
    }
    let navButtons = document.querySelectorAll(".nav-button");
    navButtons.forEach(button => {
        if(button.id=="home"){
            button.classList.add("active");
        }
        button.onclick = function () {
            button.classList.add("active");
            navButtons.forEach(button2 => {
                if (button2.id != button.id) {
                    console.log("remove"+button2.id);
                    button2.classList.remove("active");
                }
            })
            nav.setPage(button.id);
        }
    })

    nav.setPage("home"); //set the page to home
    native.get_mods().then(modList => {
        modList.forEach(mod => {
            console.log(mod.description);
        })
    })
})