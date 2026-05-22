import os
import re

assets_dir = "app/assets"
found_assets = set()

# Regex to find references to assets/something.js or assets/something.css
pattern = re.compile(r'assets/[a-zA-Z0-9_-]+\.(?:js|css)')

for file in os.listdir(assets_dir):
    if file.endswith(".js"):
        path = os.path.join(assets_dir, file)
        try:
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
            matches = pattern.findall(content)
            for m in matches:
                found_assets.add("/" + m)
        except Exception as e:
            print(f"Error reading {file}: {e}")

print(f"Found {len(found_assets)} asset references in total:")
for asset in sorted(list(found_assets)):
    print(f"  {asset}")

# Let's save the comprehensive list
with open("chunks_list.txt", "w", encoding="utf-8") as f:
    for asset in sorted(list(found_assets)):
        f.write(asset + "\n")
