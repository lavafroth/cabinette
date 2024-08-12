import json
from glob import glob

root = {
    'classes': set(),
    'annotations': [],
}

for file in glob('*.json'):
    with open(file) as handle:
        merger = json.load(handle)
        root['classes'] = root['classes'].union(merger['classes'])
        root['annotations'].extend(merger['annotations'])

print(root['classes'])
root['classes'] = list(root['classes'])
with open('annotations.json', 'w') as handle:
    json.dump(root, handle)
