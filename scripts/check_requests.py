import os

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

# Let's search for "config.driveall.cn" and print 200 characters before and after it
target = "config.driveall.cn"
index = 0
while True:
    index = js_content.find(target, index)
    if index == -1:
        break
    start = max(0, index - 300)
    end = min(len(js_content), index + len(target) + 300)
    print(f"\n--- MATCH AT INDEX {index} ---")
    print(js_content[start:end])
    index += len(target)
