with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

# Let's search for "wc=" or "wc =" or "const wc"
targets = ["const wc", "wc=", "wc ="]
for target in targets:
    idx = content.find(target)
    if idx != -1:
        print(f"Found '{target}' at index {idx}:")
        snippet = content[max(0, idx - 100):min(len(content), idx + 2000)]
        safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
        print(safe_snippet)
        print("=" * 80)
