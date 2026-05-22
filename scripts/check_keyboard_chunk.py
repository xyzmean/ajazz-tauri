import os

with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

target = "3141-32825-AK980"
index = js_content.find(target)
if index != -1:
    start = max(0, index - 200)
    end = min(len(js_content), index + len(target) + 200)
    print("AK980 match context:")
    print(js_content[start:end])
else:
    print("AK980 not found!")
