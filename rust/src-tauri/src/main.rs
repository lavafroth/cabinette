#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Error as E, Result};
use candle_nn::VarBuilder;
use candle_token_classification::BertLikeTokenClassificationHead;
use candle_token_classification::BertTokenClassificationHead;
use candle_token_classification::BILOU;
use candle_transformers::models::bert::DTYPE;
use std::sync::OnceLock;
use tokenizers::Tokenizer;

static MODEL: OnceLock<ModelWithParams> = OnceLock::new();

#[tauri::command]
fn infer(contents: &str) -> String {
    let model = MODEL.get().unwrap();
    println!("{contents}");
    let entity_groups = model
        .model
        .classify(
            contents,
            &model.labels,
            &model.tokenizer,
            &model.model.device,
        )
        .unwrap();

    println!("{:?}", entity_groups);

    let outputs: Vec<_> = entity_groups
        .into_iter()
        .filter_map(|eg| (eg.label != BILOU::I("O".to_string())).then_some(eg.text))
        // TODO: remove this collect
        .collect();

    format!("{:?}", outputs)
}

pub struct ModelWithParams {
    model: BertTokenClassificationHead,
    tokenizer: Tokenizer,
    labels: Vec<String>,
}

fn build_model_and_tokenizer() -> Result<ModelWithParams> {
    let device = candle_core::Device::Cpu;

    // TODO: inline the entire model's memoryview into the resulting binary
    let Some(model_dir) = glob::glob("../../trained_v*_safetensors")?
        .into_iter()
        .next()
        .and_then(|path| path.ok())
    else {
        return Err(E::msg(
            "unable to find any model directory matching \"trained_v*_safetensors\"",
        ));
    };
    let config_filename = model_dir.join("config.json");
    let tokenizer_filename = model_dir.join("tokenizer.json");
    let weights_filename = model_dir.join("model.safetensors");

    let config = std::fs::read_to_string(config_filename)?;
    let config = serde_json::from_str(&config)?;
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? };
    let classifier = BertTokenClassificationHead::load(vb, &config)?;

    // create an ordered list of labels for the chosen classifier
    let labels = config.id2label.values().cloned().collect();

    Ok(ModelWithParams {
        model: classifier,
        tokenizer,
        labels,
    })
}

fn main() -> Result<()> {
    let model_with_params = build_model_and_tokenizer()?;
    MODEL
        .set(model_with_params)
        .map_err(|_| E::msg("failed to set model singleton"))?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![infer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
