with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

target = "\u6240\u6709\u8def\u5f84\u90fd\u672a\u627e\u5230\u914d\u7f6e\u6587\u4ef6" # 所有路径都未找到配置文件
idx = content.find(target)
if idx != -1:
    print(f"Found at index {idx}:")
    start = max(0, idx - 1000)
    end = min(len(content), idx + 1000)
    snippet = content[start:end]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)
else:
    print("Not found!")
