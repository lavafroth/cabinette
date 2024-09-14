## Cabinette

A WIP ingredient inventory management app with an actually useful AI.

## Goals

In terms of decreasing priority.

- [ ] Release weights
- [x] Unified UI
- [ ] Static builds
- [ ] Streamline training pipeline

# UI (WIP)

We're using Tauri to package very minimal HTML, TailwindCSS and bits of ECMAScript
into a desktop app (and a mobile app in the future).

![dark](/assets/dark.png)
![light](/assets/light.png)

## Try it out

### Development

Using Nix is the officially supported way of development. We use direnv to automatically
enable a flake when entering the project directory. Make sure to have nix and direnv. Then run

```
direnv allow
```

### Train the model

- Run `jupyter-lab`.
- Open the `bert_named_entity_recongition.ipynb` notebook.
- Run all the cells.
- You now have a directory called trained_v`X`_safetensors where `X` is some version.

### Performing inference in Rust

- Change into the `rust` directory
- Execute `cargo tauri dev`
- Click on the plus icon in the top right (or bottom right on mobile devices)
- Type your recipe instructions in the text area

You will now see the inference from the text in the top bar.

For example, "I will bake a cake with those eggs" results in the following:
 
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
