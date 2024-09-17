import * as native from "./native.js"
import * as localize from "./localize.js"
let selectedPathEl
let nextBtnEl

function get_better_path(path_str) {
  if (path_str.substr(-1) == native.get_sep()) {
    return path_str
  } else {
    return path_str + native.get_sep();
  }
}


async function greet() {
  let path = await native.open_file_chooser(true);
  if (!await native.is_a_vaild_game_path(path)) {
    await native.open_message_box("Finality Framework",await native.get_text("oobe.invaild_game_path"), "error")
    //selectedPathEl.value = "未选择游戏路径"
  } else {
    selectedPathEl.value = path
  }
}

async function confirm_next() {
  selectedPathEl.value = get_better_path(selectedPathEl.value);
  if (!await native.is_a_vaild_game_path(selectedPathEl.value)) {
    native.open_message_box("Finality Framework",await native.get_text("oobe.invaild_game_path_please_check"), "error")
    selectedPathEl.value = ""
  } else {
    await native.set_oobe_over(true)
    await native.save_config()
    await native.extract_bootstrap()
    window.location.href = "./main.html"
  }
}
window.addEventListener("DOMContentLoaded", async () => {
  localize.refresh_locale_text();
  if (!await native.jre_exists()) {
    native.open_message_box("Finality Framework", await native.get_text("oobe.jre_not_found"), "error")
    native.open_website("https://www.java.com")
    native.exit_program();
  }
  if (await native.is_oobe_over()) {
    window.location.href = "./main.html"
  }
  let chooseLangEl = document.querySelector("#choose_lang")
  native.get_language().then((value) => { chooseLangEl.value = value })
  chooseLangEl.addEventListener("change", async function (event) {
    await native.set_lang(event.target.value);
    location.reload();
  })
  nextBtnEl = document.querySelector("#nextBtn");
  nextBtnEl.onclick = function () { confirm_next() }
  selectedPathEl = document.querySelector("#selectedPath");
  selectedPathEl.addEventListener("keydown", function (event) {
    if (event.keyCode == 13) {
      confirm_next();
      event.preventDefault();
    }
  });
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

window.addEventListener("load", () => {
  setTimeout(() => {
    document.querySelector("#loading-mask").style.opacity = "0";
  }, 150)
});