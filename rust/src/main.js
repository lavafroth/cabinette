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
  let chipletsEl = document.querySelector("#chiplets");
  let chiplets = inference.map((i) => generateElements(`<div class="chiplet">${i}</div>`)[0]);
  chipletsEl.replaceChildren(...chiplets);
}

async function ingredients(filter = null) {
  var ingrs_raw = await invoke("ingredients");
  if (filter !== null) {
    ingrs_raw = ingrs_raw.filter((i) => i.includes(filter));
  }
  let ingredientsEl = document.querySelector("#ingredients");
  let ingrs = ingrs_raw.map((i) => generateElements(`<div class="card">${i}</div>`)[0]);
  ingredientsEl.replaceChildren(...ingrs);
}

window.addEventListener("DOMContentLoaded", () => {

  const plusIconEl = document.querySelector('#plus-icon');
  const mainPageEl = document.querySelector('#main-page');
  const editPageEl = document.querySelector('#edit-page');
  const discardIconEl = document.querySelector('#discard-icon');
  const createIconEl = document.querySelector('#create-icon');
  const recipeNameEl = document.querySelector('#recipe-name');

  ingredients();

  const searchBoxEl = document.querySelector('#search');
  searchBoxEl.addEventListener('keyup',(e)=>{
    ingredients(e.target.value)
  });

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
