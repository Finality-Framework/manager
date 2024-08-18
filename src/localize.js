import * as native from "./native.js"

export function refresh_locale_text() {
  let pEls = document.querySelectorAll("p");
  pEls.forEach(p => {
    native.get_text(p.textContent).then((result) => { p.textContent = result })
  })
  let h1Els = document.querySelectorAll("h1");
  h1Els.forEach(h1 => {
    native.get_text(h1.textContent).then((result) => { h1.textContent = result })
  })
  let inputEls = document.querySelectorAll("input");
  inputEls.forEach(inputEl => {
    native.get_text(inputEl.placeholder).then((result) => { inputEl.placeholder = result })
  })
  let btnEls = document.querySelectorAll("button");
  btnEls.forEach(btn => {
    native.get_text(btn.textContent).then((result) => { btn.textContent = result })
  })
}

window.addEventListener("DOMContentLoaded", () => {
  refresh_locale_text();
})