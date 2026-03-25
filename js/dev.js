document.addEventListener("DOMContentLoaded", processList)
function processList() {
  const list = document.querySelectorAll("main li")
  const p = document.querySelectorAll("main li p")

  p.forEach(item => {
    if (!item.innerHTML.startsWith("[-]")) return

    item.innerHTML = "<input type=checkbox data-status=processing />" + item.innerHTML.slice("[-]".length)
  })

  list.forEach(item => {
    if (item.querySelector("input[type=checkbox]") !== null
      || !item.innerHTML.startsWith("[-]")) return

    item.innerHTML = "<input type=checkbox data-status=processing />" + item.innerHTML.slice("[-]".length)
  })
}
