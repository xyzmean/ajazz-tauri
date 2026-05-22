# ajazz-tauri — Ajazz driver (Tauri + Vue 3)

Автономный драйвер клавиатур Ajazz на **Tauri (Rust) + Vue 3**, владеющий и
протоколом, и UI — без скрейпленного апстрим-бандла и без WebHID.

## Почему такая форма

Апстрим-приложение и наш реверс-эксперимент опираются на **WebHID**
(`navigator.hid`), которого в вебвью Tauri нет (WebKitGTK / WebView2 / WKWebView
не поддерживают WebHID). Поэтому весь HID-обмен живёт в **Rust** через крейт
`hidapi`, а фронтенд на Vue общается с ним через Tauri `invoke()`.

Сам протокол портирован из **`ajazz-driver-reverse`** (`src/protocol/core.ts`) в
`src-tauri/src/protocol.rs` (опкоды, кадрирование 0xAA/0x55, чанк-транспорт,
декод `getDeviceInfo`).

## Привязка к `ajazz-driver-reverse` (по коммиту)

Таблица моделей — не дублируется руками: она берётся из репозитория
`ajazz-driver-reverse`, запиненного по конкретному коммиту.

- **`.reverse-rev`** — SHA коммита `ajazz-driver-reverse`, на который опирается сборка.
- CI чекаутит этот репозиторий на SHA в `.reverse-src/` и копирует оттуда
  `models.json` → `src-tauri/models.json` перед сборкой (см.
  `.github/workflows/build-tauri.yml`).
- Закоммиченный `src-tauri/models.json` соответствует текущему пину; обновление
  протокола/моделей — это осознанный bump `.reverse-rev` + ре-синк.

```
dev-tauri/
├── .reverse-rev              пин коммита ветки reverse (источник models.json)
├── src/                      Vue 3 + Vite фронтенд (адаптивный, без min-width гейта)
│   ├── App.vue               список устройств + карточка device-info
│   └── lib/api.ts            обёртки invoke() (единственный мост к Rust)
└── src-tauri/                Rust-бэкенд
    ├── src/protocol.rs       порт HID-протокола (транспорт на hidapi)
    ├── src/commands.rs       Tauri-команды: list_devices, get_device_info
    ├── src/models.rs         поиск модели по встроенному models.json
    └── models.json           таблица из 42 моделей (синхронизируется из reverse@.reverse-rev)
```

## Сборка и запуск

```bash
npm install
npm run tauri dev      # запуск приложения (Vite dev-сервер + Rust)
npm run tauri build    # упаковка нативного бинарника
npm run build          # только фронтенд (vue-tsc type-check + vite)
```

Зависимости сборки под Linux: `libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev
libudev-dev` (плюс обычный build-essential). На Windows используется WebView2
(WebHID не нужен — `hidapi` работает с HID-стеком ОС напрямую).

## Статус

- [x] Порт протокола на Rust: кадрирование, чанк-чтение, `get_device_info`
- [x] Tauri-команды `list_devices` / `get_device_info`; поиск модели
- [x] Vue UI: список устройств + инфо, адаптивно (одна колонка < 680px)
- [x] Таблица моделей пинится из ветки `reverse` по коммиту (`.reverse-rev`)
- [ ] Проверка на реальном железе (транспорт исходит из 32-байтных репортов / report id 0)
- [ ] Больше фич: game mode, подсветка, раскладка, Rapid Trigger (кодеры есть в `reverse`)
- [ ] i18n (переиспользовать апстрим `langs/*.json`), рендер раскладки, изображения
