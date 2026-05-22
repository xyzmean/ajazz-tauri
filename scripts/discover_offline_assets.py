import os
import re
import urllib.request
import urllib.parse

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

# 1. Extract all keyboardImg values
img_pattern = re.compile(r'keyboardImg:\s*"([^"]+)"')
kb_images = sorted(list(set(img_pattern.findall(content))))

# 2. Extract all keyboardOutline values
outline_pattern = re.compile(r'keyboardOutline:\s*"([^"]+)"')
kb_outlines = sorted(list(set(outline_pattern.findall(content))))

# 3. Extract other static asset files like key_switch.png, kb_bg.png
other_statics = ["keyboards/kb_bg.png", "img/key_switch.png"]

print(f"Found {len(kb_images)} keyboard images:")
for img in kb_images[:10]:
    print(f"  - {img}")
if len(kb_images) > 10:
    print(f"  ... and {len(kb_images) - 10} more.")

print(f"\nFound {len(kb_outlines)} keyboard outline images:")
for out in kb_outlines[:10]:
    print(f"  - {out}")
if len(kb_outlines) > 10:
    print(f"  ... and {len(kb_outlines) - 10} more.")

# Create cache directory
CACHE_DIR = "app/cache"
os.makedirs(os.path.join(CACHE_DIR, "static/keyboards"), exist_ok=True)
os.makedirs(os.path.join(CACHE_DIR, "static/outline"), exist_ok=True)
os.makedirs(os.path.join(CACHE_DIR, "static/img"), exist_ok=True)
os.makedirs(os.path.join(CACHE_DIR, "config/img"), exist_ok=True)

# Function to download with retry and error print
def download_file(url, local_path):
    os.makedirs(os.path.dirname(local_path), exist_ok=True)
    if os.path.exists(local_path):
        # Already downloaded
        return True
    print(f"Downloading: {url} -> {local_path}")
    try:
        urllib.request.urlretrieve(url, local_path)
        return True
    except Exception as e:
        print(f"  Failed: {e}")
        return False

# Download keyboard images
print("\n--- Downloading Keyboard Images ---")
for img in kb_images:
    # They are in keyboards/
    url = f"https://static.driveall.cn/static/keyboards/{img}"
    local = os.path.join(CACHE_DIR, "static/keyboards", img)
    download_file(url, local)

# Download keyboard outlines
print("\n--- Downloading Keyboard Outlines ---")
for out in kb_outlines:
    url = f"https://static.driveall.cn/static/outline/{out}"
    local = os.path.join(CACHE_DIR, "static/outline", out)
    download_file(url, local)

# Download key_switch.png and kb_bg.png
print("\n--- Downloading Other Statics ---")
download_file("https://config.driveall.cn/img/key_switch.png", os.path.join(CACHE_DIR, "config/img/key_switch.png"))
download_file("https://static.driveall.cn/static/keyboards/kb_bg.png", os.path.join(CACHE_DIR, "static/keyboards/kb_bg.png"))

print("\nAsset discovery and caching completed!")
