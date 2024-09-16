import * as native from './native.js';
import * as localize from './localize.js'
window.addEventListener('DOMContentLoaded', async () => {
    if(await native.is_oobe_over()){
        window.location.href = "./main.html";
    }else{
        localize.useSystemLanguage();
        window.location.href = "./oobe.html";
    }
})