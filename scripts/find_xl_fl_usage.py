import os
import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

# Search for xl(..., fl(..., ol(
for func in ['xl', 'fl', 'ol']:
    pattern = re.compile(rf'{func}\(([^)]+)\)')
    matches = pattern.finditer(js_content)
    
    print(f"\n=== USAGES OF {func}() ===")
    count = 0
    for match in matches:
        start = max(0, match.start() - 150)
        end = min(len(js_content), match.end() + 150)
        print(f"Match {count + 1} at index {match.start()}:")
        # Try to print but safely avoid encoding issues
        snippet = js_content[start:end]
        safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
        print(f"  {safe_snippet}")
        count += 1
        if count >= 10:
            print("  ... and more matches truncated")
            break
