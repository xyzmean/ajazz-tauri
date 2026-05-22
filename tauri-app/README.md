# tauri-app/

Standalone Ajazz keyboard driver: **Tauri (Rust) + Vue 3**, owning both the
protocol and the UI — no scraped upstream bundle, no WebHID.

## Why this shape

The upstream app and our `reverse-source/` experiment both rely on **WebHID**
(`navigator.hid`), which Tauri webviews do not provide (WebKitGTK / WebView2 /
WKWebView have no WebHID). So all HID communication lives in **Rust** via the
`hidapi` crate, and the Vue frontend talks to it through Tauri `invoke()`.

The protocol itself is ported from `reverse-source/src/protocol/core.ts` to
`src-tauri/src/protocol.rs` (opcodes, 0xAA/0x55 framing, chunked transport,
`getDeviceInfo` decode). The model table is generated from the same source.

```
tauri-app/
├── src/                      Vue 3 + Vite frontend (adaptive, no min-width gate)
│   ├── App.vue               device list + device-info view
│   └── lib/api.ts            invoke() wrappers (the only bridge to Rust)
└── src-tauri/                Rust backend
    ├── src/protocol.rs       HID protocol port (hidapi transport)
    ├── src/commands.rs       Tauri commands: list_devices, get_device_info
    ├── src/models.rs         model lookup from embedded models.json
    └── models.json           42-model table (generated from reverse-source)
```

## Develop & build

```bash
npm install
npm run tauri dev      # run the app (Vite dev server + Rust)
npm run tauri build    # package a native binary
npm run build          # frontend only (vue-tsc type-check + vite)
```

Linux build deps: `libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev libudev-dev`
(plus the usual build-essential). Windows uses WebView2 (no WebHID needed —
`hidapi` talks to the OS HID stack directly).

## Status

- [x] Rust protocol port: framing, chunked read transport, `get_device_info`
- [x] Tauri commands `list_devices` / `get_device_info`; model lookup
- [x] Vue UI: device list + info, responsive (single-column under 680px)
- [ ] Verified against real hardware (transport assumes 32-byte reports / report id 0)
- [ ] More features: game mode, lighting, keymap, Rapid Trigger (encoders exist in reverse-source)
- [ ] i18n (reuse upstream `langs/*.json`), keyboard layout rendering, images
