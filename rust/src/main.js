const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

function generateElements(html) {
  const template = document.createElement('template');
  template.innerHTML = html.trim();
  return template.content.children;
}

async function infer(text) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let raw_inference = await invoke("infer", { contents: text });
  let inference = JSON.parse(raw_inference);
  chipletsEl = document.querySelector("#chiplets");
  chiplets = inference.map((i) => generateElements(`<div class="chiplet">${i}</div>`)[0]);
  chipletsEl.replaceChildren(...chiplets);
}

window.addEventListener("DOMContentLoaded", () => {
  chipletsEl = document.querySelector("#chiplets");

  const plusIconEl = document.querySelector('#plus-icon');
  const mainPageEl = document.querySelector('#main-page');
  const editPageEl = document.querySelector('#edit-page');
  const discardIconEl = document.querySelector('#discard-icon');
  const createIconEl = document.querySelector('#create-icon');
  const recipeNameEl = document.querySelector('#recipe-name');

  plusIconEl.addEventListener('click', (e) => {
    mainPageEl.classList.toggle('m-fadeOut');
    mainPageEl.classList.toggle('m-fadeIn');
    editPageEl.classList.toggle('m-fadeIn');
    editPageEl.classList.toggle('m-fadeOut');
  });

  discardIconEl.addEventListener('click', (e) => {
    editPageEl.classList.toggle('m-fadeOut');
    editPageEl.classList.toggle('m-fadeIn');
    mainPageEl.classList.toggle('m-fadeIn');
    mainPageEl.classList.toggle('m-fadeOut');
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

  createIconEl.addEventListener('click', (e) => {
    editPageEl.classList.toggle('m-fadeOut');
    editPageEl.classList.toggle('m-fadeIn');
    mainPageEl.classList.toggle('m-fadeIn');
    mainPageEl.classList.toggle('m-fadeOut');
    console.log(`${recipeNameEl.value} ${recipeInstEl.value}`);
  })
});
