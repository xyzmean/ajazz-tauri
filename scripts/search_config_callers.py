with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

# Search for "加载键盘" (in unicode escape)
target = "\u52a0\u8f7d\u952e\u76d8"
idx = content.find(target)
if idx != -1:
    print(f"Found target at index {idx}.")
    func_idx = content.rfind("async function ", 0, idx)
    if func_idx != -1:
        func_name_part = content[func_idx + len("async function "):func_idx + len("async function ") + 20]
        func_name = func_name_part.split("(")[0].strip()
        print(f"Enclosing function name is: '{func_name}'")
        
        print(f"Searching for calls to '{func_name}(':")
        call_idx = 0
        call_count = 1
        while True:
            call_idx = content.find(f"{func_name}(", call_idx)
            if call_idx == -1:
                break
            # Clean snippet for printing
            snippet = content[max(0, call_idx - 150):min(len(content), call_idx + 150)]
            safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
            print(f"Call {call_count} at index {call_idx}:")
            print(safe_snippet.strip())
            print("-" * 50)
            call_idx += len(func_name) + 1
            call_count += 1
else:
    print("Target not found!")
