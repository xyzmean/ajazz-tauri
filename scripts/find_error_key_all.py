import os

assets_dir = "app/assets"
target = "message_driverall_error"

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
                    print(f"Found in {file} at index {idx}:")
                    start = max(0, idx - 200)
                    end = min(len(content), idx + 200)
                    print(content[start:end])
                    print("-" * 50)
                    idx += len(target)
        except Exception as e:
            print(f"Error reading {file}: {e}")
