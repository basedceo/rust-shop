const slidingMenu = document.getElementById("slideMenu");
const hamMenu = document.getElementById("hamMenu");
const closebtn = document.getElementById("close");
// For sproduct.html
const mainImg = document.getElementById("mainImg");
const smallImg = document.getElementsByClassName("small-img");
// For Shoppage
const shopProduct = document.getElementsByClassName("pro");


hamMenu.addEventListener("click", () => {
    slidingMenu.classList.add("active");
});

closebtn.addEventListener("click", () => {
    slidingMenu.classList.remove("active");
});

// Appling onclink on the small-images

for (const i of smallImg) {
    i.addEventListener("click", (e) => {
        mainImg.src = e.target.src;
    });
}

// Appling onclink on the Products

for (let i = 0; i <= 3; i++) {

    if (shopProduct[i]) {
        shopProduct[i].addEventListener("click", (e) => {
            window.location.href = 'sproduct.html'
        });
    }
}