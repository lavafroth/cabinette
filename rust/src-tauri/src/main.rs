#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Error as E, Result};
use candle_token_classification::BILOU;
use std::sync::OnceLock;
mod model;
mod store;
use crate::model::ModelWithParams;
use crate::store::Store;
use std::fs;

static MODEL: OnceLock<ModelWithParams> = OnceLock::new();
static STORE: OnceLock<Store> = OnceLock::new();

#[tauri::command]
fn infer(contents: &str) -> String {
    let model = MODEL.get().unwrap();
    println!("{contents}");
    let entity_groups = model.classify(contents).unwrap();

    println!("{:?}", entity_groups);

    let outputs: Vec<_> = entity_groups
        .into_iter()
        .filter_map(|eg| (eg.label != BILOU::I("O".to_string())).then_some(eg.text))
        // TODO: remove this collect
        .collect();

    format!("{:?}", outputs)
}

#[tauri::command]
fn ingredients() -> Vec<String> {
    let store = STORE.get().unwrap();
    store.ingredients.clone()
}

fn main() -> Result<()> {
    let filename = "stor.cabinette";
    let glorb = fs::read(filename).unwrap();
    let store: Store = bincode::deserialize(&glorb)?;
    STORE
        .set(store)
        .map_err(|_| E::msg("failed to retrieve recipes from store"))?;

    let model_with_params = model::build_model_and_tokenizer()?;
    MODEL
        .set(model_with_params)
        .map_err(|_| E::msg("failed to set model singleton"))?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![infer, ingredients])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
