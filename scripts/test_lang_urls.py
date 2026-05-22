import urllib.request
import json

urls_to_test = [
    "https://ajazz.driveall.cn/langs/en.json",
    "https://config.driveall.cn/langs/en.json",
    "https://static.driveall.cn/langs/en.json",
    "https://static.driveall.cn/static/langs/en.json",
    "https://config.driveall.cn/static/langs/en.json",
    "https://ajazz.driveall.cn/assets/langs/en.json",
    "https://ajazz.driveall.cn/static/langs/en.json",
    "https://config.driveall.cn/logo/ajazz/langs/en.json"
]

print("Probing CDN URLs for en.json...")

for url in urls_to_test:
    print(f"Testing: {url}")
    try:
        req = urllib.request.Request(
            url, 
            headers={'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'}
        )
        response = urllib.request.urlopen(req, timeout=5)
        content_type = response.headers.get("content-type", "")
        print(f"  Status: {response.status}, Content-Type: {content_type}")
        
        # Read a bit
        data = response.read()
        
        # Check if it is valid JSON
        try:
            parsed = json.loads(data.decode('utf-8'))
            print("  -> VALID JSON FOUND!")
            print(f"  -> Snippet: {str(parsed)[:200]}")
            break
        except Exception as je:
            print(f"  -> Not valid JSON: {je}")
    except Exception as e:
        print(f"  -> Error: {e}")
