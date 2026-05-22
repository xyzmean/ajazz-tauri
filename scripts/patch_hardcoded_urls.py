import os
import urllib.request

# 1. Download the logo locally so it renders offline
logo_url = "https://config.driveall.cn/logo/ajazz/logo.png"
logo_local_path = "app/cache/config/logo/ajazz/logo.png"
os.makedirs(os.path.dirname(logo_local_path), exist_ok=True)

if not os.path.exists(logo_local_path):
    print(f"Downloading logo: {logo_url} -> {logo_local_path}")
    try:
        urllib.request.urlretrieve(logo_url, logo_local_path)
        print("  Success!")
    except Exception as e:
        print(f"  Failed to download logo: {e}")

# 2. Patch hardcoded URLs in layout-default-m1PKe3cP.js
js_path = "app/assets/layout-default-m1PKe3cP.js"

with open(js_path, "r", encoding="utf-8") as f:
    content = f.read()

# Hardcoded keyboard images
old_kb_img_url = "https://static.driveall.cn/static/keyboards/"
new_kb_img_url = "./cache/static/keyboards/"

# Hardcoded logo
old_logo_url = "https://config.driveall.cn/logo/ajazz/logo.png"
new_logo_url = "./cache/config/logo/ajazz/logo.png"

patched = False
if old_kb_img_url in content:
    content = content.replace(old_kb_img_url, new_kb_img_url)
    print("Patched hardcoded keyboard image URLs!")
    patched = True

if old_logo_url in content:
    content = content.replace(old_logo_url, new_logo_url)
    print("Patched hardcoded logo URL!")
    patched = True

if patched:
    with open(js_path, "w", encoding="utf-8") as f:
        f.write(content)
    print("Successfully saved patched JS file!")
else:
    print("No hardcoded URLs to patch or already patched.")
