import os
import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

# Let's search for "he = async" (or "he=async") and then search for calls to "he(" or "he.apply" or similar.
# In the previous search, we saw:
# "sa.setItem("led_item_"+t.ledConfig.localforageKey+"_"+Y,Q)}d.value=X},he=async(L,X)=>{const Y=xl(`/json/${L}/${X}.json`),Q=await fetch(Y);if(!Q.ok)throw new Error(`Failed to fetch ${Y}`);return await Q.json()}"
# Let's find where "he" is called!
# Usually "he(" is followed by two arguments: he(arg1, arg2)

calls = re.findall(rf'he\(([^,]+),([^)]+)\)', js_content)
print(f"Found {len(calls)} calls to he():")
for c in calls[:10]:
    print(f"  - he({c[0].strip()}, {c[1].strip()})")

# Let's search for "/json/" or "he(" context and print around it to understand the loop of Y/X
target = "he=async"
idx = js_content.find(target)
if idx != -1:
    print(f"\nContext around he definition:")
    snippet = js_content[max(0, idx - 400):min(len(js_content), idx + len(target) + 400)]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)
