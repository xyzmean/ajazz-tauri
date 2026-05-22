with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

target = "\u52a0\u8f7d\u952e\u76d8"
idx = content.find(target)
if idx != -1:
    func_idx = content.rfind("async function ", 0, idx)
    if func_idx != -1:
        func_name_part = content[func_idx + len("async function "):func_idx + len("async function ") + 20]
        func_name = func_name_part.split("(")[0].strip()
        print(f"Function name is: '{func_name}'")
        
        call_idx = 0
        call_count = 1
        while True:
            call_idx = content.find(f"{func_name}(", call_idx)
            if call_idx == -1:
                break
            if call_count <= 6:
                snippet = content[max(0, call_idx - 150):min(len(content), call_idx + 150)]
                safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
                print(f"Call {call_count} at index {call_idx}:")
                print(safe_snippet.strip())
                print("-" * 50)
            call_idx += len(func_name) + 1
            call_count += 1
