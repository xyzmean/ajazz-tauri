import os
import re

assets_dir = "app/assets"
target = "ru-BdFY45Te.js"

for file in os.listdir(assets_dir):
    if file.endswith(".js"):
        path = os.path.join(assets_dir, file)
        try:
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
            idx = 0
            while True:
                idx = content.find(target, idx)
                if idx == -1:
                    break
                print(f"Found {target} in {file} at index {idx}:")
                snippet = content[max(0, idx - 150):min(len(content), idx + 150)]
                safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
                print(safe_snippet.strip())
                print("-" * 50)
                idx += len(target)
        except Exception as e:
            print(f"Error reading {file}: {e}")
