with open("app/assets/layout-default-m1PKe3cP.js", "r", encoding="utf-8") as f:
    js_content = f.read()

# Let's search for "Xs(" or "function Xs" or similar
# Let's look at the context around "Go(s)" where Xs(s,"i18n") is called
target = "Go(s)"
idx = js_content.find(target)
if idx != -1:
    print("Found Go(s)! Printing context:")
    snippet = js_content[max(0, idx - 100):min(len(js_content), idx + 1000)]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)

# Let's search for the function Xs definition
xs_idx = js_content.find("function Xs")
if xs_idx != -1:
    print("\nFound function Xs! Printing definition:")
    snippet = js_content[xs_idx:xs_idx + 600]
    safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
    print(safe_snippet)
else:
    # Try finding "Xs="
    xs_idx2 = js_content.find("Xs=")
    if xs_idx2 != -1:
        print("\nFound Xs=! Printing context:")
        snippet = js_content[xs_idx2 - 100:xs_idx2 + 600]
        safe_snippet = "".join([c if ord(c) < 128 else f"\\u{ord(c):04x}" for c in snippet])
        print(safe_snippet)
