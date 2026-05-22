with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

target = "he("
idx = 0
match_num = 1
while True:
    idx = content.find(target, idx)
    if idx == -1:
        break
    print(f"\n=== Call {match_num} at index {idx} ===")
    snippet = content[max(0, idx - 200):min(len(content), idx + 200)]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)
    idx += len(target)
    match_num += 1
