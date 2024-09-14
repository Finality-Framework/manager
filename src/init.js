import * as native from './native.js';
window.addEventListener('DOMContentLoaded', async () => {
    if(await native.is_oobe_over()){
        window.location.href = "./main.html";
    }else{
        window.location.href = "./oobe.html";
    }
})