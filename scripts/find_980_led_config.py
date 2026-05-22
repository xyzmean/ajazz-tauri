import os
import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

# The AK980 MAX configs were:
# bn={vendorId:3141,productId:32835,supplierName:"ajazz",name:"AK980 MAX",...,keyboardConfig:wn}
# zn={vendorId:3141,productId:32849,supplierName:"ajazz",name:"AK980 MAX",...,keyboardConfig:On}
# Zn={vendorId:3141,productId:32852,supplierName:"ajazz",name:"AK980 MAX",...,keyboardConfig:Xn}

# Let's search for definitions of wn, On, Xn.
targets = ['wn', 'On', 'Xn']

for t in targets:
    print(f"\nSearching definition of {t}:")
    var_def_pattern = re.compile(rf'(?:const|let|var)\s+{t}\s*=')
    match = var_def_pattern.search(js_content)
    if match:
        start_idx = match.start()
        # Print next 1200 characters to see the object structure
        snippet = js_content[start_idx:start_idx + 1200]
        safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
        print(f"Definition found:\n{safe_snippet}")
    else:
        var_def_pattern2 = re.compile(rf'{t}\s*=')
        match2 = var_def_pattern2.search(js_content)
        if match2:
            start_idx = match2.start()
            snippet = js_content[start_idx:start_idx + 1200]
            safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
            print(f"Definition found (fallback):\n{safe_snippet}")
        else:
            print("Definition not found!")
