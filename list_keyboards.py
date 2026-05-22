import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

# Let's find all occurrences of name: "..." or name: '...'
# and check if 'vendorId' or 'productId' is near it (within 150 chars).
matches = re.finditer(r'name\s*:\s*["\']([^"\']+)["\']', content)

kb_names = set()
for m in matches:
    name = m.group(1)
    start_pos = m.start()
    # Look at the window of 150 characters before this match
    window = content[max(0, start_pos - 150):start_pos]
    if "vendorId" in window or "productId" in window:
        # Exclude generic labels to get only true models
        if name not in [
            "USB Keyboard", "Gaming Keyboard", "AJAZZ KEYBOARD", "Keyboard", "Default",
            "2.4G Dongle", "HS USB Dongle", "AJAZZ Keyboard", "gW997mgkb"
        ]:
            kb_names.add(name)

unique_names = sorted(list(kb_names))
print(f"Detected {len(unique_names)} true keyboard models:")
with open("keyboards_list.txt", "w", encoding="utf-8") as out:
    for name in unique_names:
        print(f"  - {name}")
        out.write(f"- {name}\n")
