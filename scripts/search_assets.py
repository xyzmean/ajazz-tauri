import os
import re

APP_DIR = "app"
patterns = [
    r'https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(?:/[^\s"]*)?',
    r'/config/[^\s"]*'
]

compiled_patterns = [re.compile(p) for p in patterns]

print("Searching JavaScript files for URLs and configurations...")

for root, dirs, files in os.walk(APP_DIR):
    for file in files:
        if file.endswith(".js"):
            file_path = os.path.join(root, file)
            print(f"\nAnalyzing: {file_path}")
            try:
                with open(file_path, "r", encoding="utf-8") as f:
                    content = f.read()
                    
                    found = set()
                    for cp in compiled_patterns:
                        matches = cp.findall(content)
                        for m in matches:
                            found.add(m)
                            
                    # Print found items
                    for item in sorted(found):
                        # Filter out common library URLs if too long, or print a subset
                        if len(item) < 150:
                            print(f"  Found: {item}")
            except Exception as e:
                print(f"  Error reading file: {e}")
