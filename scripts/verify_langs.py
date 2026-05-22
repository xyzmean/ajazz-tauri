import os
import json

langs_dir = "app/langs"
for file in os.listdir(langs_dir):
    path = os.path.join(langs_dir, file)
    if os.path.isfile(path):
        try:
            with open(path, "r", encoding="utf-8") as f:
                json.load(f)
            print(f"[OK] {file} is valid JSON.")
        except Exception as e:
            print(f"[ERROR] {file} is INVALID JSON! Error: {str(e)}. Deleting...")
            try:
                os.remove(path)
            except Exception as del_err:
                print(f"Failed to delete {file}: {str(del_err)}")
