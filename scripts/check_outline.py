import os
import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

# Let's search for keyboardOutline inside On and Xn
# On is defined as: On={vendorId:3141,productId:32848,supplierName:"ajazz",name:"AK980 MAX",...,keyboardConfig:On}
# Wait, On={... keyboardConfig:On }? Oh, On in the module exports was:
# Un=Object.freeze(Object.defineProperty({__proto__:null,keyboardConfig:On},Symbol.toStringTag,{value:"Module"})),zn={vendorId:3141,productId:32849,supplierName:"ajazz",name:"AK980 MAX",...,keyboardConfig:On}
# This means the inner configuration object is On (for zn) or wn (for bn) or Xn (for Zn).
# Let's search for "keyboardOutline" or "outline" in layout-default-m1PKe3cP.js near "On=" or "Xn=" or "wn="

for var_name in ['On', 'Xn', 'wn']:
    print(f"\nSearching for 'outline' or 'keyboardOutline' inside/near {var_name}:")
    # Let's find the definition of the variable (e.g. "On={") and extract the next 5000 characters
    match = re.search(rf'\b{var_name}\s*=\s*\{{', js_content)
    if match:
        start = match.start()
        # Find the matching closing bracket or just grab 8000 characters (minified, so it's long)
        snippet = js_content[start:start + 15000]
        # Let's search for 'outline' in this snippet
        outlines = re.findall(r'["\']?keyboardOutline["\']?\s*:\s*["\']([^"\']+)["\']', snippet)
        print(f"  Found outlines: {outlines}")
    else:
        print("  Not found definition.")
