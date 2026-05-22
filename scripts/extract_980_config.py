import os
import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

# Let's search for the definition of the AK980 MAX.
# We saw: "/src/config/keyboards/ajazz/3141-32835-AK980 MAX.ts": Kn
# Let's find "Kn =" or where Kn is defined.
# Vite usually outputs like: const Kn = { ... }; or Kn = { ... } or similar.
# Let's do a regex search for something that defines Kn.
# Since JS is minified, let's search for "AK980 MAX.ts" and look at the mapping value: Kn.
# Then let's find the definition of Kn in the file.

# Let's find the exact variable names mapped to all AK980 models:
ak980_patterns = re.findall(r'"/src/config/keyboards/ajazz/3141-\d+-AK980[^"]+":([a-zA-Z0-9_$]+)', js_content)
print(f"AK980 models mapped variables: {ak980_patterns}")

# Let's print the definitions of these variables.
# Vite variables are often declared as "const Kn={...}" or "let Kn={...}" or "Kn={..."
for var_name in set(ak980_patterns):
    print(f"\nSearching definition of variable: {var_name}")
    # Search for "const var_name=" or "let var_name=" or "var var_name="
    # or just "var_name=" with boundaries
    var_def_pattern = re.compile(rf'(?:const|let|var)\s+{var_name}\s*=')
    match = var_def_pattern.search(js_content)
    if match:
        start_idx = match.start()
        # Print next 1000 characters to see the object structure
        snippet = js_content[start_idx:start_idx + 1500]
        safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
        print(f"Definition found:\n{safe_snippet}")
    else:
        # Try finding var_name={
        var_def_pattern2 = re.compile(rf'{var_name}\s*=')
        match2 = var_def_pattern2.search(js_content)
        if match2:
            start_idx = match2.start()
            snippet = js_content[start_idx:start_idx + 1500]
            safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
            print(f"Definition found (fallback):\n{safe_snippet}")
        else:
            print("Definition not found via regex!")
