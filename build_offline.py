import os
import re
import urllib.request
import urllib.parse

BASE_URL = "https://ajazz.driveall.cn"
CONFIG_CDN = "https://config.driveall.cn"
STATIC_CDN = "https://static.driveall.cn"

APP_DIR = "app"
ASSETS_DIR = os.path.join(APP_DIR, "assets")
LANGS_DIR = os.path.join(APP_DIR, "langs")
CACHE_DIR = os.path.join(APP_DIR, "cache")

os.makedirs(ASSETS_DIR, exist_ok=True)
os.makedirs(LANGS_DIR, exist_ok=True)
os.makedirs(CACHE_DIR, exist_ok=True)

# Helper function to request remote resource with a real Browser User-Agent
def request_url(url):
    req = urllib.request.Request(
        url,
        headers={'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'}
    )
    with urllib.request.urlopen(req, timeout=15) as response:
        return response.read()

def download_file(url, local_path):
    os.makedirs(os.path.dirname(local_path), exist_ok=True)
    try:
        data = request_url(url)
        with open(local_path, "wb") as f:
            f.write(data)
        print(f"  [OK] Saved {os.path.basename(local_path)}")
        return True
    except Exception as e:
        print(f"  [FAIL] {url} -> {e}")
        return False

print("1. Scraping entrypoint index.html from remote...")
try:
    html_data = request_url(BASE_URL + "/")
    html_str = html_data.decode("utf-8")
    
    # Save index.html locally
    with open(os.path.join(APP_DIR, "index.html"), "w", encoding="utf-8") as f:
        f.write(html_str)
    print("  [OK] Saved index.html")
except Exception as e:
    print(f"Fatal error fetching index.html: {e}")
    exit(1)

print("\n2. Discovering and downloading core assets from HTML...")
# Find scripts and stylesheets
asset_paths = re.findall(r'href="(/assets/[^"]+)"|src="(/assets/[^"]+)"', html_str)
assets = sorted(list(set([p[0] or p[1] for p in asset_paths if p[0] or p[1]])))

layout_file = None
for asset in assets:
    local_path = os.path.join(APP_DIR, asset.lstrip("/"))
    download_file(BASE_URL + asset, local_path)
    
    # Keep track of layout file which holds all configurations and dynamic imports
    if "layout-default" in asset and asset.endswith(".js"):
        layout_file = local_path

if not layout_file:
    print("Warning: layout-default.js not found in index.html, using fallback search...")
    # fallback to searching all assets
    for root, dirs, files in os.walk(ASSETS_DIR):
        for file in files:
            if "layout-default" in file and file.endswith(".js"):
                layout_file = os.path.join(root, file)
                break

print(f"  Using layout-default file: {layout_file}")

print("\n3. Discovering and downloading relative dynamic JavaScript imports...")
all_discovered_chunks = set()

if layout_file and os.path.exists(layout_file):
    with open(layout_file, "r", encoding="utf-8") as f:
        layout_content = f.read()
    
    # 3.1 Find all dynamic import("./...") or import('./...')
    imports = re.findall(r'import\(\s*["\']\./([^"\']+)["\']\s*\)', layout_content)
    for imp in imports:
        all_discovered_chunks.add(imp)
        
    # 3.2 Find all strings referencing assets/filename.js (like in Vite chunk maps)
    assets_js_refs = re.findall(r'assets/[a-zA-Z0-9_-]+\.js', layout_content)
    for ref in assets_js_refs:
        filename = ref.split("/")[-1]
        all_discovered_chunks.add(filename)

# Also check other JS files in ASSETS_DIR for assets/... references
for root, dirs, files in os.walk(ASSETS_DIR):
    for file in files:
        if file.endswith(".js"):
            try:
                with open(os.path.join(root, file), "r", encoding="utf-8") as f:
                    file_content = f.read()
                assets_js_refs = re.findall(r'assets/[a-zA-Z0-9_-]+\.js', file_content)
                for ref in assets_js_refs:
                    filename = ref.split("/")[-1]
                    all_discovered_chunks.add(filename)
            except Exception:
                pass

print(f"  Found {len(all_discovered_chunks)} dynamic chunks and assets in JS files.")
for chunk in sorted(list(all_discovered_chunks)):
    download_file(f"{BASE_URL}/assets/{chunk}", os.path.join(ASSETS_DIR, chunk))

print("\n4. Downloading Russian, English, and other localized languages...")
LANGS = ['ar', 'de', 'en', 'es', 'fr', 'id', 'it', 'ja', 'ko', 'pt-BR', 'pt', 'ru', 'th', 'vi', 'zh-TW', 'zh']
for lang in LANGS:
    lang_url = f"{CONFIG_CDN}/langs/{lang}.json"
    download_file(lang_url, os.path.join(LANGS_DIR, f"{lang}.json"))

print("\n5. Discovering and pre-caching keyboard images for all models...")
if layout_file and os.path.exists(layout_file):
    kb_images = re.findall(r'keyboardImg:\s*"([^"]+)"', layout_content)
    unique_images = sorted(list(set(kb_images)))
    print(f"  Found {len(unique_images)} keyboard models.")
    for img in unique_images:
        img_url = f"{STATIC_CDN}/static/keyboards/{img}"
        download_file(img_url, os.path.join(CACHE_DIR, "static/keyboards", img))

print("\n6. Downloading standalone core brand logos and switch graphics...")
download_file(f"{CONFIG_CDN}/img/key_switch.png", os.path.join(CACHE_DIR, "config/img/key_switch.png"))
download_file(f"{STATIC_CDN}/static/keyboards/kb_bg.png", os.path.join(CACHE_DIR, "static/keyboards/kb_bg.png"))
download_file(f"{CONFIG_CDN}/logo/ajazz/logo.png", os.path.join(CACHE_DIR, "config/logo/ajazz/logo.png"))

print("\n7. Patching minified JavaScript bundles for offline relative routing...")
if layout_file and os.path.exists(layout_file):
    with open(layout_file, "r", encoding="utf-8") as f:
        js_content = f.read()

    # Regex search and replace for dynamic config, static and layout functions
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

    # Patch hardcoded keyboard static cdn images and logos
    patched_content = patched_content.replace("https://static.driveall.cn/static/keyboards/", "./cache/static/keyboards/")
    patched_content = patched_content.replace("https://config.driveall.cn/logo/ajazz/logo.png", "./cache/config/logo/ajazz/logo.png")

    with open(layout_file, "w", encoding="utf-8") as f:
        f.write(patched_content)
    print("  [OK] Successfully patched JavaScript relative pathways.")

print("\nOffline build generation completed successfully!")
