import os
import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

# Vite dynamic import map typically looks like:
# "/config/keyboards/ajazz/3141-32825-AK980": [ "assets/chunk-name.js", ... ] or similar.
# Let's search for all strings starting with "assets/" and ending with ".js" inside the main file.
chunk_regex = re.compile(r'"assets/[a-zA-Z0-9_-]+\.js"')
matches = chunk_regex.findall(content)

# Also let's search for "assets/chunk-name.js" without quotes
chunk_regex_2 = re.compile(r'assets/[a-zA-Z0-9_-]+\.js')
matches_2 = chunk_regex_2.findall(content)

all_chunks = set(matches + matches_2)
cleaned_chunks = []
for c in all_chunks:
    c_clean = c.replace('"', '').replace("'", "")
    if c_clean.startswith("assets/"):
        cleaned_chunks.append("/" + c_clean)

cleaned_chunks = sorted(list(set(cleaned_chunks)))
print(f"Found {len(cleaned_chunks)} potential lazy-loaded JS chunks:")
for c in cleaned_chunks[:20]:
    print(f"  {c}")
if len(cleaned_chunks) > 20:
    print(f"  ... and {len(cleaned_chunks) - 20} more.")

# Write these to a file so we can download them
with open("chunks_list.txt", "w", encoding="utf-8") as f:
    for c in cleaned_chunks:
        f.write(c + "\n")
