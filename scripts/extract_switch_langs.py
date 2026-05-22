with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

target = "ru-BdFY45Te.js"
idx = content.find(target)
if idx != -1:
    print("Found language import switch context:")
    start = max(0, idx - 500)
    end = min(len(content), idx + 500)
    snippet = content[start:end]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)
else:
    print("Not found!")
