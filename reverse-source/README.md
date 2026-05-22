# reverse-source/

Рабочая папка реверс-инжиниринга: цель — **владеть HID-протоколом клавиатур Ajazz**
независимо от апстрим-бандла.

Текущее приложение (`../app/`) — это скрейпленная и пропатченная по URL копия
апстрим WebHID-приложения: чёрный ящик, привязанный к минифицированному бандлу,
который непредсказуемо переминифицируется при каждой пересборке. Эта папка —
начало *настоящего порта*: документированный типизированный слой протокола, поверх
которого можно строить любой фронтенд (web, CLI, Tauri, кроссплатформенный) без
зависимости от апстрим-бандла.

## Структура

```
reverse-source/
├── package.json / tsconfig.json   тулинг сборки (tsc → dist/)
├── extract_models.py              регенерирует src/protocol/models.ts из бандла
├── index.core.min.js              дословная копия ../app/assets/index-BosuIawc.js (ядро 47 КБ)
├── index.core.pretty.js           то же через Prettier (2545 строк) — для справки
└── src/
    ├── main.ts                    Electron-хост: отдаёт СУЩЕСТВУЮЩИЙ UI из ../app, авто-грант WebHID
    └── protocol/
        ├── core.ts                константы, enum CMD, сборка/разбор кадра, чанк-транспорт
        ├── commands.ts            кодеры/декодеры по фичам (device info, game mode, клавиши, LED, RT, сброс)
        ├── models.ts              таблица из 42 моделей (авто-генерация)
        └── index.ts               баррель
```

## Сборка и запуск

```bash
npm install
npm run build          # tsc → dist/   (это и есть «сборка»)
npm start              # сборка + запуск Electron (использует UI из ../app)
npm run package        # отдельный Windows .exe через electron-packager
npm run extract:models # перегенерировать models.ts после пересборки апстрима
```

## Методология

1. **Sourcemaps проверены — их нет.** На любой запрос `*.js.map` CDN отдаёт
   SPA-заглушку `index.html` (896 байт, `content-type: text/html`) — «мягкий» 200,
   а не настоящий мап. Поэтому исходник реконструируется из минифицированного кода,
   а не восстанавливается из мапов.
2. **Prettier** — чтобы сделать минифицированный чанк навигируемым.
3. **Финальный блок `export {…}` апстрима — розеттский камень.** Он сопоставляет
   каждый минифицированный символ человеческому имени (см. таблицу ниже). Вместе с
   enum'ом имён команд и китайскими debug-строками (пережили минификацию) протокол
   читается почти напрямую.

## Формат кадра (полная спецификация — в шапке `core.ts`)

- WebHID, report ID `0`. Длина репорта берётся из дескриптора устройства (по умолчанию 32 байта).
- Magic запроса `0xAA`, ответа `0x55`. Заголовок 8 байт; payload с байта 8.
- Адрес — little-endian в байтах [3..4]. Многобайтные передачи чанкуются
  (`ceil(contentSize / (reportSize - 8))` пакетов) с ретраями и таймаутом на каждый пакет.
- **Универсальной контрольной суммы нет** — несколько команд (например, `SET_MUSIC_DATA`)
  добавляют свою в последний байт; это относится к самим командам, а не к транспорту.

## Карта символов (минифицированное → смысл)

Пере-выводить после каждой пересборки апстрима — имена меняются.

| Мин | Имя | | Мин | Имя |
|-----|-----|-|-----|-----|
| `E` | `CMD` (enum опкодов) | | `C` | `readDataChunks` (транспорт) |
| `pe` | `FACTORY_RESET_TYPE` | | `P` | сборщик пакета (внутр.) |
| `He` | `deviceInit` | | `Pe` | парсер ответа (внутр.) |
| `Ce` | `getDeviceInfo` | | `j` | матчер ответа (внутр.) |
| `Re` | `getGameMode` | | `z` | открытие устройства (внутр.) |
| `Ne` | `getKeyData` | | `qe` | `setGameMode` |
| `Ue` | `getLEDEffect` | | `lt` | `setKeyData` |
| `ke` | `getCustomLEDData` | | `mt` | `setLEDEffect` |
| `be` | `getLightBox` | | `dt` | `setCustomLEDData` |
| `Ge` | `getMacroData` | | `yt` | `setLightBox` |
| `Fe` | `getMagneticAxisRT` | | `ht` | `setMacroData` |
| `Ve` | `getMagneticAxisDKSData` | | `Dt` | `setMagneticAxisRT` |
| `ve` | `getFnKeyData` | | `It` | `setMagneticAxisDKSData` |
| `Ke` | `getDefaultFnKeyMatrix` | | `ft` | `setFnKeyData` |
| `it` | `getSingleKeyData` | | `Ae` | `factoryReset` |
| `ut` | `getFnSingleKeyData` | | `ct` | `resetAll` |
| `ot` | `getMagneticAxisStatus` | | `st` | `clearCalibration` |
| `gt`/`pt`/`St` | `getAllLightsRGB[24G][64Byte]` | | `Lt` | `clearLedData` |
| `Qe`/`tt` | `startCalibration[V2]` | | `Tt`/`_t` | `setMusicData[V1]` |
| `et`/`rt` | `stopCalibration[V2]` | | `Et` | `setGifLighting` |
| `nt`/`at` | `start/stopSimulationTest` | | `At` | `setDotMatrixMode` |
| `We` | `startResetListener` | | `Mt` | `setLedSyncAnimation` |
| `vt` | `startDeviceStateListener` | | `wt` | `setLedUserAnimation` |
| `Je`/`Ze`/`je` | слушатели 24G disconnect/sleep/wake | | `Ct` | `setLedDateTime` |
| | | | `Ot`/`Nt`/`Kt`/`Rt` | `setTft*` (builtIn/dateTime/screenInfo/userAnimation) |

## Статус

- [x] Проверено наличие sourcemaps (их нет)
- [x] Ядро деминифицировано в читаемый вид
- [x] `core.ts`: константы, enum'ы `CMD`/`FactoryResetType`, сборка/разбор кадра,
      чанк-транспорт, матчер ответа, открытие устройства, рабочий пример `getDeviceInfo`
- [x] `commands.ts`: кодеры/декодеры для game mode, раскладки (raw), LED-эффекта,
      Rapid Trigger (магнитная ось), factory reset, парсинга калибровки
- [x] `models.ts`: таблица из 42 моделей (vendorId/productId/connectType/routesName/keyboardImg)
- [x] Electron-хост (`main.ts`) — **компилируется чисто (tsc) и стартует**: отдаёт
      существующий UI, локальный сервер поднимается, path-traversal guard проверен
- [ ] Оставшиеся команды: макросы, custom LED data, FN-клавиши, DKS, TFT/анимации, музыка
- [ ] Маппинг раскладки (raw-записи → подписи клавиш по модели)
- [ ] Свой фронтенд вместо апстрим-UI; кроссплатформенная упаковка
```

## Оговорка по безопасности

`webSecurity: false` и сплошной авто-грант WebHID унаследованы из оригинальной
обёртки и оставлены сознательно (нужны для WebHID + локального кэша). Весь
обслуживаемый контент локальный и доверенный, но это осознанный компромисс.
