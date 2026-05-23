// Builds the background helper (ajazz-helperd) and stages it as a Tauri sidecar.
//
// Tauri's `bundle.externalBin` expects the binary to exist as
// `src-tauri/binaries/ajazz-helperd-<target-triple>(.exe)`; at bundle/dev time Tauri strips the
// triple and places it next to the app as `ajazz-helperd(.exe)` — which is what the GUI's
// `helperd_path()` looks for. Run from `beforeDevCommand`/`beforeBuildCommand` so both local dev
// and CI builds always have the sidecar present (Tauri errors if externalBin is missing).
const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "..");
const manifest = path.join(root, "src-tauri", "Cargo.toml");

const hostLine = execSync("rustc -Vv").toString().match(/^host:\s*(.+)$/m);
if (!hostLine) throw new Error("could not determine host target triple from `rustc -Vv`");
const triple = hostLine[1].trim();

const exe = process.platform === "win32" ? ".exe" : "";
const dir = path.join(root, "src-tauri", "binaries");
fs.mkdirSync(dir, { recursive: true });
const dst = path.join(dir, `ajazz-helperd-${triple}${exe}`);

// Bootstrap: tauri-build's externalBin check runs during the helperd's own compile and refuses to
// proceed if the staged sidecar is missing. Touch a placeholder first so the first build succeeds,
// then overwrite with the real artifact.
if (!fs.existsSync(dst)) {
  fs.writeFileSync(dst, "");
  console.log(`[stage-helper] wrote placeholder ${path.relative(root, dst)}`);
}

console.log("[stage-helper] building ajazz-helperd (release)...");
execSync(`cargo build --release --manifest-path "${manifest}" --bin ajazz-helperd`, {
  stdio: "inherit",
});

const src = path.join(root, "src-tauri", "target", "release", `ajazz-helperd${exe}`);
fs.copyFileSync(src, dst);
console.log(`[stage-helper] staged ${path.relative(root, dst)}`);
