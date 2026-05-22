with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

target = "message_driverall_error"
index = 0
match_num = 1
while True:
    index = js_content.find(target, index)
    if index == -1:
        break
    print(f"\n=== MATCH {match_num} AT INDEX {index} ===")
    start = max(0, index - 300)
    end = min(len(js_content), index + 300)
    snippet = js_content[start:end]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)
    index += len(target)
    match_num += 1

if match_num == 1:
    print("message_driverall_error NOT found in JS files. Checking translation JSONs...")
    # Let's check translation json files for it
    import json
    import os
    langs_dir = "app/langs"
    for file in os.listdir(langs_dir):
        if file.endswith(".json"):
            path = os.path.join(langs_dir, file)
            try:
                with open(path, "r", encoding="utf-8") as jf:
                    data = json.load(jf)
                    # Search recursively in keys or values
                    def search_dict(d, search_key):
                        results = []
                        if isinstance(d, dict):
                            for k, v in d.items():
                                if k == search_key:
                                    results.append((k, v))
                                results.extend(search_dict(v, search_key))
                        elif isinstance(d, list):
                            for item in d:
                                results.extend(search_dict(item, search_key))
                        return results
                    
                    found = search_dict(data, "message_driverall_error")
                    if found:
                        print(f"Found in {file}: {found}")
            except Exception as e:
                print(f"Error reading {file}: {e}")
