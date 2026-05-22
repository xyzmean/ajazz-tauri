import os

assets_dir = "app/assets"
targets = ["driverall", "_error", "message_"]
output_lines = []

for file in os.listdir(assets_dir):
    if file.endswith(".js"):
        path = os.path.join(assets_dir, file)
        try:
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
            for target in targets:
                idx = 0
                count = 0
                while True:
                    idx = content.find(target, idx)
                    if idx == -1:
                        break
                    count += 1
                    snippet = content[max(0, idx - 150):min(len(content), idx + 150)]
                    # Clean the snippet to be ascii safe
                    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
                    output_lines.append(f"File: {file} | Target: {target} | Match {count} at index {idx}:")
                    output_lines.append(safe_snippet.strip())
                    output_lines.append("-" * 80)
                    idx += len(target)
                    if count >= 30: # Limit to 30 matches to keep it clean
                        break
        except Exception as e:
            output_lines.append(f"Error reading {file}: {str(e)}")

with open("search_patterns_output.txt", "w", encoding="utf-8") as out_f:
    out_f.write("\n".join(output_lines))

print("Done! Output written to search_patterns_output.txt")
