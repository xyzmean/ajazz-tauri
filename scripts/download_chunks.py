import os
import urllib.request
import urllib.parse

BASE_URL = "https://ajazz.driveall.cn"
OUTPUT_DIR = "app/assets"

os.makedirs(OUTPUT_DIR, exist_ok=True)

with open("chunks_list.txt", "r", encoding="utf-8") as f:
    chunks = [line.strip() for line in f if line.strip()]

print(f"Downloading {len(chunks)} chunks from remote server...")

for chunk in chunks:
    # Ensure we resolve the path correctly (e.g. /assets/index-BosuIawc.js)
    clean_chunk = chunk.lstrip("/")
    # Get just the filename
    filename = os.path.basename(clean_chunk)
    local_path = os.path.join(OUTPUT_DIR, filename)
    
    # Check if already exists and is not empty
    if os.path.exists(local_path) and os.path.getsize(local_path) > 100:
        print(f"[SKIP] {filename} already exists locally.")
        continue
        
    remote_url = BASE_URL + chunk
    print(f"Downloading: {remote_url} -> {local_path} ...")
    try:
        req = urllib.request.Request(
            remote_url,
            headers={'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'}
        )
        with urllib.request.urlopen(req, timeout=10) as response:
            data = response.read()
            with open(local_path, "wb") as local_f:
                local_f.write(data)
        print("  Success!")
    except Exception as e:
        print(f"  Failed: {e}")

print("Chunk downloading completed!")
