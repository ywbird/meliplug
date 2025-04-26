

const flips = document.getElementsByClassName("flip");
[...flips].forEach((flip)=>{
    flip.addEventListener("click", () => {
	flip.classList.toggle("fliped");
    })
})
