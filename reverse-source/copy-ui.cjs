/**
 * Copy the generated UI (repo-root ../app) into ./app-ui so electron-packager
 * bundles a self-contained app. Run after build_offline.py has produced ../app.
 * Cross-platform (uses fs.cpSync, Node 16.7+).
 */
const fs = require("fs");
const path = require("path");

const src = path.resolve(__dirname, "..", "app");
const dest = path.resolve(__dirname, "app-ui");

if (!fs.existsSync(path.join(src, "index.html"))) {
  console.error(`UI source not found at ${src}. Run "python build_offline.py" at the repo root first.`);
  process.exit(1);
}

fs.rmSync(dest, { recursive: true, force: true });
fs.cpSync(src, dest, { recursive: true });
console.log(`Copied UI: ${src} -> ${dest}`);
