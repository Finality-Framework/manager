export function setPage(pageName){
    pageName = pageName + "-page";
    let pages = document.querySelectorAll("div.page")
    pages.forEach(page => {
        if(page.id == pageName){
            page.style.display = "block"
        }else{
            page.style.display = "none"
        }
    })
}