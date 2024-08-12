use std::path::Path;

use candle_transformers::models::bert::DTYPE;
use itertools::Itertools;

use anyhow::{Error as E, Result};
use candle_nn::VarBuilder;
use candle_token_classification::BertLikeTokenClassificationHead; // Import the token classifier trait from this library
use candle_token_classification::BertTokenClassificationHead;
use clap::Parser;
use tokenizers::Tokenizer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable tracing (generates a trace-timestamp.json file).
    #[arg(long)]
    tracing: bool,

    /// When set, compute embeddings for this prompt.
    #[arg(long)]
    prompt: Option<String>,
}

impl Args {
    fn build_model_and_tokenizer(
        &self,
    ) -> Result<(BertTokenClassificationHead, Tokenizer, Vec<String>)> {
        let device = candle_core::Device::Cpu;

        let Some(model_dir) = glob::glob("../trained_v*_safetensors")?
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

        let vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? };
        let classifier = BertTokenClassificationHead::load(vb, &config)?;

        // create an ordered list of labels for the chosen classifier
        let labels = config
            .id2label
            .iter()
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, label)| label.to_string())
            .collect();

        Ok((classifier, tokenizer, labels))
    }
}

fn main() -> Result<()> {
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;

    let args = Args::parse();
    let _guard = if args.tracing {
        println!("tracing...");
        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };

    let (model, tokenizer, labels) = args.build_model_and_tokenizer()?;

    if let Some(prompt) = args.prompt {
        let output = model.classify(
            // classify some text (or use `model.forward` to get the output tensor)
            &prompt,
            &labels,
            &tokenizer,
            &model.device,
        )?;
        println!("{output:#?}");
    }
    Ok(())
}
