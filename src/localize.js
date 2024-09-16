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

export function useSystemLanguage(){
  switch (navigator.language) {
    case "zh-CN":
      native.set_lang("zh_cn");
      break;
    case "zh-TW":
      native.set_lang("zh_tw");
      break;
    case "en-US":
      native.set_lang("en_us");
      break;
    default:
      native.set_lang("en_us");
  }
}