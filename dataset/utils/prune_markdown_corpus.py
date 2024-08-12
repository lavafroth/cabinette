from glob import glob
from pathlib import Path
import os

files = map(Path, glob("content/*.md"))
csv_dir = Path('pruned')
os.makedirs(csv_dir, exist_ok=True)
for file in files:
    out_filename = csv_dir / Path(file.stem + ".txt")
    with open(file) as f:
        c = f.read()
    start = c.index("## Directions")
    c = c[start:]
    lines = c.splitlines()
    lines = lines[1:]  # ignore the directions line

    pruned_lines = []
    for line in lines:
        words = line.split()
        if len(words) == 0:
            continue
        if words[0][0] in '1234567890':
            words = words[1:]
            line = ' '.join(words)
            pruned_lines.append(line)
    with open(out_filename, "w") as handle:
        handle.writelines('\n'.join(pruned_lines))
