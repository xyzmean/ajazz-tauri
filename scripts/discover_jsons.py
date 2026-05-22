import os
import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

# Let's search for "localforageKey" and see what values it gets assigned,
# or look for "ledConfig" structures.
# An ledConfig object typically looks like: { localforageKey: "some_key", ... } or similar.
# Let's write a regex to find: localforageKey:\s*"([^"]+)"
keys_pattern = re.compile(r'localforageKey:\s*"([^"]+)"')
keys = sorted(list(set(keys_pattern.findall(content))))

print(f"Found {len(keys)} localforageKeys (LED configuration IDs):")
for k in keys:
    print(f"  - {k}")

# What about GIF effect ids?
# We saw: gifEffectIds: [...] or similar.
# Let's search for gifEffectIds or just ids under gif-lighting
gif_id_pattern = re.compile(r'gifEffectIds:\s*\[([^\]]+)\]')
gif_ids = gif_id_pattern.findall(content)
print(f"\nFound gifEffectIds chunks:")
for gid in gif_ids:
    print(f"  - {gid}")

# Let's search for any strings matching /json/ or /gif-lighting/ or /gif/
json_paths = re.findall(r'["\']/json/[^"\']+["\']', content)
print(f"\nExplicit json paths found in code: {json_paths}")
