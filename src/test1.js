const { invoke } = window.__TAURI__.tauri
const { open } = window.__TAURI__.dialog
let selectedPathEl
let selected
async function greet() {
  selectedPathEl.textContent = await invoke("load_mod",{ name: "/home/user/testMod" })
}

window.addEventListener("DOMContentLoaded", () => {
  selectedPathEl = document.querySelector("#selectedPath");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
