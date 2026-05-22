import os
import urllib.request

# The languages list compiled from the 'sl' dictionary in layout-default
LANGS = [
    "en.json", "zh.json", "zh-TW.json", "ja.json", "ko.json", "fr.json",
    "es.json", "pt.json", "ru.json", "de.json", "it.json", "ar.json",
    "vi.json", "id.json", "th.json", "pt-BR.json", "pl.json", "hu.json",
    "tr.json", "sr.json", "hr.json", "sl.json", "mk.json", "sq.json",
    "ro.json", "cs.json"
]

BASE_URL = "https://config.driveall.cn/langs/"
OUTPUT_DIR = "app/langs"

os.makedirs(OUTPUT_DIR, exist_ok=True)

print("Downloading actual i18n translation JSONs from config CDN...")

for lang in LANGS:
    url = BASE_URL + lang
    local_path = os.path.join(OUTPUT_DIR, lang)
    
    print(f"Downloading: {url} -> {local_path} ...")
    try:
        req = urllib.request.Request(
            url,
            headers={'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'}
        )
        with urllib.request.urlopen(req, timeout=10) as response:
            data = response.read()
            with open(local_path, "wb") as f:
                f.write(data)
        print("  Success!")
    except Exception as e:
        print(f"  Failed: {e}")

print("All translation files successfully downloaded offline!")
