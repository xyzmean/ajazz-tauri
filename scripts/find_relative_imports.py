import re

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    content = f.read()

# Find all occurrences of import("./...") or import('./...')
imports_1 = re.findall(r'import\(\s*["\']\./([^"\']+)["\']\s*\)', content)

# Let's clean and print
all_imports = sorted(list(set(imports_1)))
print(f"Found {len(all_imports)} relative dynamic imports:")
for imp in all_imports:
    print(f"  ./{imp}")

# Append these to chunks_list.txt as /assets/filename
with open("chunks_list.txt", "r", encoding="utf-8") as f:
    existing_chunks = [line.strip() for line in f if line.strip()]

new_chunks = set(existing_chunks)
for imp in all_imports:
    new_chunks.add("/assets/" + imp)

with open("chunks_list.txt", "w", encoding="utf-8") as f:
    for chunk in sorted(list(new_chunks)):
        f.write(chunk + "\n")

print("\nUpdated chunks_list.txt with relative dynamic imports!")
