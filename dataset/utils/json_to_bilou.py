from spacy.training import offsets_to_biluo_tags
import json
import spacy
from glob import glob
import csv

nlp = spacy.blank('en')

with open('pytorch_dataset.csv', 'w') as handle:
    writer = csv.DictWriter(handle, ('sentence', 'word_labels'), quoting=csv.QUOTE_ALL)
    writer.writeheader()
    for file in glob('annotations/*.json'):
        with open(file) as handle:
            contents = json.load(handle)
        anns = contents['annotations']
        for ann in anns:
            text = ann[0]
            labels = ann[1]['entities']
            doc = nlp(text)
            tags = offsets_to_biluo_tags(doc, labels)
            writer.writerow({
                'sentence': text,
                'word_labels': ','.join(tags)
            })
