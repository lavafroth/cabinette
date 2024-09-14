use anyhow::{Error as E, Result};
use candle_nn::VarBuilder;
use candle_token_classification::BertLikeTokenClassificationHead;
use candle_token_classification::BertTokenClassificationHead;
use candle_token_classification::EntityGroup;
use candle_transformers::models::bert::DTYPE;

use tokenizers::Tokenizer;
pub struct ModelWithParams {
    model: BertTokenClassificationHead,
    tokenizer: Tokenizer,
    labels: Vec<String>,
}

impl ModelWithParams {
    pub fn classify<'a>(&self, contents: &'a str) -> candle_core::Result<Vec<EntityGroup<'a>>> {
        self.model
            .classify(contents, &self.labels, &self.tokenizer, &self.model.device)
    }
}

pub fn build_model_and_tokenizer() -> Result<ModelWithParams> {
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
