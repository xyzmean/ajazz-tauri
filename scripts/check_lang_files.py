with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

target = "jo("
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
