import os
import re
import urllib.request
import urllib.parse

BASE_URL = "https://ajazz.driveall.cn/"
OUTPUT_DIR = "app"

os.makedirs(OUTPUT_DIR, exist_ok=True)
os.makedirs(os.path.join(OUTPUT_DIR, "assets"), exist_ok=True)

# Fetch index.html
print("Fetching index.html...")
response = urllib.request.urlopen(BASE_URL)
html_content = response.read().decode('utf-8')

# Write index.html
with open(os.path.join(OUTPUT_DIR, "index.html"), "w", encoding="utf-8") as f:
    f.write(html_content)

# Extract asset URLs
assets = []
# Match scripts, preloads, and stylesheets
asset_patterns = [
    r'src="(/assets/[^"]+)"',
    r'href="(/assets/[^"]+)"'
]

for pattern in asset_patterns:
    matches = re.findall(pattern, html_content)
    assets.extend(matches)

# Deduplicate
assets = list(set(assets))

print(f"Found {len(assets)} assets to download: {assets}")

# Download each asset
for asset in assets:
    # Remove leading slash for local saving path
    local_path = os.path.join(OUTPUT_DIR, asset.lstrip("/"))
    # Form absolute remote URL
    remote_url = urllib.parse.urljoin(BASE_URL, asset)
    
    print(f"Downloading {remote_url} -> {local_path} ...")
    try:
        urllib.request.urlretrieve(remote_url, local_path)
    except Exception as e:
        print(f"Failed to download {remote_url}: {e}")

print("Assets download completed!")
