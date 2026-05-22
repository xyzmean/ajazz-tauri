import os

assets_dir = "app/assets"
target = "driveall.cn"

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
                snippet = content[max(0, idx - 100):min(len(content), idx + 100)]
                safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
                print(f"Found in {file} at index {idx}:")
                print(safe_snippet.strip())
                print("-" * 50)
                idx += len(target)
        except Exception as e:
            print(f"Error reading {file}: {e}")
