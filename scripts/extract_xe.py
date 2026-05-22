with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

idx = 558742
start = max(0, idx - 1500)
end = min(len(content), idx + 1000)
snippet = content[start:end]
safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
print(safe_snippet)
