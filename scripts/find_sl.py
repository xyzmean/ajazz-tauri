with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

target = "function Xs"
idx = js_content.find(target)
if idx != -1:
    print("Found function Xs! Printing 3500 characters before it:")
    # Print 3500 characters before the target
    start = max(0, idx - 3500)
    snippet = js_content[start:idx]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)
else:
    print("function Xs not found!")
