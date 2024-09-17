
import * as native from "./native.js";
import * as nav from "./nav.js"
import * as localize from "./localize.js"


window.addEventListener("DOMContentLoaded", async () => {
    await native.load_mods();
})

window.addEventListener("DOMContentLoaded", () => {
    localize.refresh_locale_text();
    native.setDecoration(true);
    let menuButtonTemplate = document.querySelector("#menu-button-template")
    class MenuButton extends HTMLElement {
        static observedAttributes = ["icon", "text"];
        constructor() {
            super()
            let shadow = this.attachShadow({ mode: 'open' })
            shadow.appendChild(menuButtonTemplate.content.cloneNode(true))
        }
        async attributeChangedCallback(name, oldValue, newValue) {
            console.log(`属性 ${name} 已由 ${oldValue} 变更为 ${newValue}。`);
            if (name == "icon") {
                console.log("icon changed to " + newValue)
                this.shadowRoot.querySelector("#icon").src = newValue;
                return;
            }
            if (name == "text") {
                console.log("text changed to " + newValue)
                this.shadowRoot.querySelector("#text").innerText = await native.get_text(newValue);
            }
        }
    }
    let modCardTemplate = document.querySelector("#mod-card-template")
    class ModCard extends HTMLElement {
        modObject; // mod object
        static observedAttributes = ["modname", "modenabled"];
        constructor() {
            super()
            let shadow = this.attachShadow({ mode: 'open' })
            shadow.appendChild(modCardTemplate.content.cloneNode(true))
        }
        attributeChangedCallback(name, oldValue, newValue) {
            console.log(`属性 ${name} 已由 ${oldValue} 变更为 ${newValue}。`);
            if (name == "modname") {
                console.log("modname changed to " + newValue)
                this.shadowRoot.querySelector("#mod-name").innerText = newValue;
                return;
            }
            if (name == "modenabled") {
                console.log("modenabled changed to " + newValue)
                this.modObject.enabled = JSON.parse(newValue);
                this.shadowRoot.querySelector("#switch").checked = JSON.parse(newValue);
                return;
            }
        }
    }
    window.customElements.define('menu-button', MenuButton)
    window.customElements.define('mod-card', ModCard)
    let nextBtnEl = document.querySelector("#launch");
    nextBtnEl.onclick = function () {
        native.launch_game();
    }
    buildModCardList();
    let navButtons = document.querySelectorAll(".nav-button");
    navButtons.forEach(button => {
        if (button.id == "home") {
            button.classList.add("active");
        }
        button.onclick = function () {
            button.classList.add("active");
            navButtons.forEach(button2 => {
                if (button2.id != button.id) {
                    console.log("remove" + button2.id);
                    button2.classList.remove("active");
                }
            })
            nav.setPage(button.id);
        }
    })
    nav.setPage("home"); //set the page to home

    let addModElement = document.querySelector("#add-mod");
    addModElement.onclick = async function () {
        await native.extract_mod_zip(await native.open_file_chooser(false));
        buildModCardList();
    }
})


async function buildModCardList() {
    await native.load_mods();
    document.querySelectorAll("mod-card").forEach(element => {
        element.remove();
    }
    )

    document.querySelector("#mod-cards-page").querySelectorAll("br").forEach(element => {
        element.remove();   
    }
    )

    native.get_mods().then(modList => {
        modList.forEach(async mod => {
            console.log(mod);
            let modCardElement = document.createElement("mod-card");
            let modPage = document.querySelector("#mod-cards-page");
            console.log(mod.name);
            modCardElement.modObject = mod;
            modCardElement.setAttribute("modname", mod.name);
            modCardElement.setAttribute("modenabled", mod.enabled)
            modCardElement.shadowRoot.querySelector("#remove").onclick = async function () {
                await native.remove_mod(modCardElement.modObject.id);
                buildModCardList();
            }
            modCardElement.shadowRoot.querySelector("#switch").onclick = async function () {
                modCardElement.setAttribute("modenabled", this.checked);
                await native.switch_mod(modCardElement.modObject.id, this.checked);
            }
            if (mod.description.trim().length < 1) {
                modCardElement.shadowRoot.querySelector("#description").innerText = await native.get_text("mods.no_description");
            } else if (mod.description.length > 23) {
                modCardElement.shadowRoot.querySelector("#description").innerText = mod.description.substring(0, 23) + "...";
            } else {
                modCardElement.shadowRoot.querySelector("#description").innerText = mod.description;
            }
            modPage.appendChild(modCardElement);
            modPage.appendChild(document.createElement("br"));
        })
    })
}


window.addEventListener("load", () => {
    setTimeout(() => {
        document.querySelector("#loading-mask").style.opacity = "0";
    }, 150)
});
