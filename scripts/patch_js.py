import os

js_path = "app/assets/layout-default-m1PKe3cP.js"

with open(js_path, "r", encoding="utf-8") as f:
    js_content = f.read()

# Original functions
orig_xl = 'function xl(s){return`https://config.driveall.cn${s.startsWith("/")?s:"/"+s}`}'
orig_ol = 'function ol(s){return`https://static.driveall.cn/static/${s.startsWith("/")?s:"/"+s}`}'
orig_fl = 'function fl(s){return`https://config.driveall.cn${s.startsWith("/")?s:"/"+s}`}'

# Let's check if they are in the content
all_found = True
for f_name, f_str in [('xl', orig_xl), ('ol', orig_ol), ('fl', orig_fl)]:
    if f_str in js_content:
        print(f"Found function {f_name} exactly as expected!")
    else:
        print(f"Function {f_name} was NOT found exactly as expected. Let's do a search...")
        all_found = False

# Safe replacement
if all_found:
    patched_content = js_content.replace(orig_xl, 'function xl(s){return`./cache/config${s.startsWith("/")?s:"/"+s}`}')
    patched_content = patched_content.replace(orig_ol, 'function ol(s){return`./cache/static${s.startsWith("/")?s:"/"+s}`}')
    patched_content = patched_content.replace(orig_fl, 'function fl(s){return`./cache/config${s.startsWith("/")?s:"/"+s}`}')
    
    with open(js_path, "w", encoding="utf-8") as f:
        f.write(patched_content)
    print("Successfully patched JS for offline caching!")
else:
    # If the formatting changed slightly, let's use regex to replace
    import re
    patched_content = re.sub(
        r'function xl\s*\(\s*s\s*\)\s*\{\s*return\s*[`\']https://config\.driveall\.cn\$\{s\.startsWith\(\s*["\']\/["\']\s*\)\s*\?\s*s\s*:\s*["\']\/["\']\s*\+\s*s\}\s*[`\']\s*\}',
        'function xl(s){return`./cache/config${s.startsWith("/")?s:"/"+s}`}',
        js_content
    )
    patched_content = re.sub(
        r'function ol\s*\(\s*s\s*\)\s*\{\s*return\s*[`\']https://static\.driveall\.cn/static/\$\{s\.startsWith\(\s*["\']\/["\']\s*\)\s*\?\s*s\s*:\s*["\']\/["\']\s*\+\s*s\}\s*[`\']\s*\}',
        'function ol(s){return`./cache/static${s.startsWith("/")?s:"/"+s}`}',
        patched_content
    )
    patched_content = re.sub(
        r'function fl\s*\(\s*s\s*\)\s*\{\s*return\s*[`\']https://config\.driveall\.cn\$\{s\.startsWith\(\s*["\']\/["\']\s*\)\s*\?\s*s\s*:\s*["\']\/["\']\s*\+\s*s\}\s*[`\']\s*\}',
        'function fl(s){return`./cache/config${s.startsWith("/")?s:"/"+s}`}',
        patched_content
    )
    
    with open(js_path, "w", encoding="utf-8") as f:
        f.write(patched_content)
    print("Successfully patched JS using regex replacement!")
