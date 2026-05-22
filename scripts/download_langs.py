import os
import urllib.request

LANGS = ["en.json", "zh_cn.json", "ru.json", "zh.json", "ru_ru.json"]
BASE_URL = "https://ajazz.driveall.cn/langs/"
OUTPUT_DIR = "app/langs"

os.makedirs(OUTPUT_DIR, exist_ok=True)

print("Starting language files download...")

for lang in LANGS:
    url = BASE_URL + lang
    local_path = os.path.join(OUTPUT_DIR, lang)
    
    print(f"Downloading: {url} -> {local_path} ...")
    try:
        urllib.request.urlretrieve(url, local_path)
        print("  Success!")
    except Exception as e:
        print(f"  Failed: {e}")

print("Language files download completed!")
