const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function infer(text) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  chipletsEl.textContent = await invoke("infer", { contents: text });
}

window.addEventListener("DOMContentLoaded", () => {
  chipletsEl = document.querySelector("#chiplets");
  // greetMsgEl = document.querySelector("#greet-msg");
  // document.querySelector("#greet-form").addEventListener("submit", (e) => {
  //   e.preventDefault();
  //   greet();
  // });

  const plusIconEl = document.querySelector('#plus-icon');
  const mainPageEl = document.querySelector('#main-page');
  const editPageEl = document.querySelector('#edit-page');
  plusIconEl.addEventListener('click', (e) => {
    mainPageEl.classList.toggle('m-fadeOut');
    mainPageEl.classList.toggle('m-fadeIn');
    editPageEl.classList.toggle('m-fadeIn');
    editPageEl.classList.toggle('m-fadeOut');
  });

  let timer;
  const waitTime = 500;
  const recipeInstEl = document.querySelector('#recipe-instructions');
  recipeInstEl.addEventListener('keyup', (e)=>{
    const text = e.currentTarget.value;
    clearTimeout(timer);
    timer = setTimeout(() => {
      infer(text);
    }, waitTime);
  })
});
