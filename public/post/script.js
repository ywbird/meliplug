


// /icons/IxCheckboxesEmpty.svg

let codeblocks = document.getElementsByClassName("codeblock");

[...codeblocks].forEach((codeblock) => {
    let btn = document.createElement("button");
    btn.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 512 512"><!-- Icon from Siemens Industrial Experience Icons by Siemens AG - https://github.com/siemens/ix-icons/blob/main/LICENSE.md --><path fill="currentColor" fill-rule="evenodd" d="M362.667 64H64v298.667h42.667v-256h256zM448 149.333H149.333V448H448zM405.333 192v213.333H192V192z" clip-rule="evenodd"/></svg>`;
    btn.classList.add("copy-btn");

    const code = codeblock.querySelector("code").innerText;

    btn.addEventListener("click", () => {
	navigator.clipboard.writeText(code);
    });
    
    codeblock.appendChild(btn);

    const rect = codeblock.getBoundingClientRect();

    if (rect.height > 500) {
	codeblock.classList.add("fold");

	let folder = document.createElement("div");
	folder.classList.add("folder");
	folder.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 512 512"><!-- Icon from Siemens Industrial Experience Icons by Siemens AG - https://github.com/siemens/ix-icons/blob/main/LICENSE.md --><path fill="currentColor" fill-rule="evenodd" d="M431.084 158.17L400.914 128L255.999 272.898L111.084 128l-30.17 30.17L256 333.255zM85.334 384h341.333v-42.667H85.333z" clip-rule="evenodd"/></svg>`;

	folder.addEventListener("click", () => {
	    codeblock.classList.remove("fold");
	    codeblock.classList.add("unfold");
	});
	
	codeblock.appendChild(folder);
    }
});
