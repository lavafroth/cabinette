## Cabinette

A WIP ingredient inventory management app with an actually useful AI.

## Goals

In terms of decreasing priority.

- [ ] WebUI
- [ ] Release weights
- [ ] Unified UI in Dart
- [ ] Static builds
- [ ] Streamline training pipeline

## Try it out

### Steps for Nix users only

Run `nix develop`

### Train the model

The weights are not released yet because I haven't annotated all of the dataset.
With every model being ~450MiB, I really don't want to speedrun getting the repo
to weigh 2 GiB in 5 commits.

That means you have to train the model yourself for now.
- Make sure to have CUDA.
- Install the dependencies from the `micromamba.yml` file.
- Run `jupyter-lab`.
- Open the `bert_named_entity_recongition.ipynb` notebook.
- Run all the cells.
- You now have a directory called trained_v`X`_safetensors where `X` is some version.

### Performing inference in Rust

- Change into the `rust` directory
- Execute `cargo r -- --prompt` followed by your prompt.

Example:

```sh
cargo r -- --prompt "I will bake a cake with those eggs."
```

Results in the following:
 
```ron
[
    EntityGroup {
        text: "I will bake a cake with those",
        start: 0,
        end: 29,
        label: I(
            "O",
        ),
    },
    EntityGroup {
        text: "eggs",
        start: 30,
        end: 34,
        label: I(
            "U-FOOD",
        ),
    },
    EntityGroup {
        text: ".",
        start: 34,
        end: 35,
        label: I(
            "O",
        ),
    },
]
```

### Acknowledgements

- [NeilsRogge's custom bert NER notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/BERT/Custom_Named_Entity_Recognition_with_BERT.ipynb)
- [based.cooking](https://github.com/lukesmithxyz/based.cooking) by Luke Smith and contributors which has been used to train the named entity recognition.
- TecoHolic's [NER annotator](https://tecoholic.github.io/ner-annotator/)

### License

AGPL3, aka to hell with corpos.
