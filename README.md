# ⌨️ AJAZZ Offline Desktop Driver / Офлайн-драйвер AJAZZ (Electron)

[English](#english) | [Русский](#русский)

---

# English

This repository provides a **100% offline, standalone desktop configuration utility** for the **Ajazz AK980 MAX** mechanical keyboard and other supported Ajazz models. 

It wraps the original WebHID assets from `https://ajazz.driveall.cn` into a native Windows desktop app built with **Electron**, enabling local customizations without relying on an active internet connection or external servers.

---

## 🌟 Key Features
- **100% Offline-Ready**: All dynamic JS files, localized Russian/English translations, and static keyboard layout images are preloaded and served from local relative directories.
- **Zero Configuration WebHID**: Custom Electron backend automatically detects and pairs your USB keyboard instantly without showing any browser connection prompts.
- **Adaptive Layout Scaling**: Launches maximized by default, with custom scale adjustments (`zoomFactor: 0.85`) so elements fit perfectly on laptop displays.
- **Automated GitHub Actions CI/CD**: You do not have to commit 200MB of static images and bundles to git. GitHub Actions builds the entire offline program from scratch and packages the executable automatically!

---

## ⌨️ Supported Keyboards

The utility robustly supports **18 keyboard models**:

*   **AK980 Series:**
    *   `AJAZZ AK980 MAX` (Tested & Confirmed)
    *   `AJAZZ AK980 PRO`
    *   `AJAZZ AK980 PRO 2.4G`
    *   `AJAZZ AK980 V2 PRO`
*   **AK820 Series:**
    *   `AK820`
    *   `AK820MAX` / `AJAZZ AK820MAX`
    *   `AK820 MAX Lightles`
    *   `820PRO`
*   **AK680 Series:**
    *   `AJAZZ AK680 MAX`
    *   `AJAZZ AK680 V2`
*   **AK870 Series:**
    *   `AK870MC`
*   **ALUX Series:**
    *   `AJAZZ ALUX75 PRO`
*   **AK0xx Series:**
    *   `AJAZZ AK029`
    *   `AJAZZ AK039`
*   **Other Supported Models:**
    *   `MJ84+`
    *   `QS87`
    *   `CSOL Keyboard`

---

## 🤖 Automated CI/CD (GitHub Actions)
This repository includes a completely automated build workflow:
1. **Triggers**:
   - **Weekly Run**: Every Sunday at midnight UTC, GitHub Actions automatically rebuilds the offline app from scratch to fetch the latest assets from the driver CDN.
   - **On Commit**: Triggered automatically on every new push to `main` or `master` branches.
   - **Manual Run**: Triggered with a single click via the **"Run workflow"** button in the GitHub Actions tab.
2. **Build Results**: Once completed, a ZIP archive containing the ready-to-run Windows executable (`AJAZZ Local Driver.exe`) is available for download in the execution summary under **Artifacts**!

---

## 🚀 How to Run Locally

### Prerequisites
1. **Node.js** (v18 or higher)
2. **Python 3.x**

### 1. Rebuild the Offline Assets
Run the unified script to scrape, download, analyze relative modules, and patch all URLs locally:
```bash
python build_offline.py
```

### 2. Run the Application
Launch the Electron program in production mode:
```bash
npm install
npm start
```
*To open the application with Developer Tools enabled for troubleshooting:*
```bash
npm run start:debug
```

### 3. Compile Standalone `.exe`
Package the application into a standalone portable folder containing `AJAZZ Local Driver.exe` under `dist/`:
```bash
npm run build
```

---

> [!IMPORTANT]
> Make sure your keyboard is connected **strictly via a USB cable** (not over Bluetooth or 2.4GHz wireless) when customizing keymaps, Rapid Trigger settings, and lighting profiles. Standard WebHID communication is only supported over a direct wire.

---

# Русский

Репозиторий содержит **100% автономную локальную программу настройки** для механической клавиатуры **Ajazz AK980 MAX** и других поддерживаемых моделей Ajazz.

Она оборачивает оригинальные WebHID-компоненты с сайта `https://ajazz.driveall.cn` в нативное Windows-приложение на **Electron**, позволяя менять настройки клавиатуры без интернета и внешних серверов.

---

## 🌟 Ключевые особенности
- **Полный офлайн-режим**: Все динамические JS-файлы, локализованные словари переводов (включая русский и английский языки) и изображения раскладок скачиваются локально.
- **WebHID без настройки**: Electron-бэкенд автоматически определяет и подключает вашу USB-клавиатуру мгновенно, минуя браузерные всплывающие окна авторизации.
- **Адаптивный интерфейс**: Программа запускается развернутой на весь экран по умолчанию со специальным масштабированием (`zoomFactor: 0.85`), чтобы все элементы интерфейса и таблицы настроек помещались даже на экранах ноутбуков.
- **Автоматизация через GitHub Actions**: Вам не нужно заливать в Git сотни мегабайт статичных изображений. GitHub Actions собирает готовый офлайн-установщик с нуля автоматически!

---

## ⌨️ Список поддерживаемых клавиатур

Утилита полностью поддерживает **18 моделей клавиатур**:

*   **Серия AK980:**
    *   `AJAZZ AK980 MAX` (Проверено и работает)
    *   `AJAZZ AK980 PRO`
    *   `AJAZZ AK980 PRO 2.4G`
    *   `AJAZZ AK980 V2 PRO`
*   **Серия AK820:**
    *   `AK820`
    *   `AK820MAX` / `AJAZZ AK820MAX`
    *   `AK820 MAX Lightles`
    *   `820PRO`
*   **Серия AK680:**
    *   `AJAZZ AK680 MAX`
    *   `AJAZZ AK680 V2`
*   **Серия AK870:**
    *   `AK870MC`
*   **Серия ALUX:**
    *   `AJAZZ ALUX75 PRO`
*   **Серия AK0xx:**
    *   `AJAZZ AK029`
    *   `AJAZZ AK039`
*   **Другие поддерживаемые модели:**
    *   `MJ84+`
    *   `QS87`
    *   `CSOL Keyboard`

---

## 🤖 Автоматическая сборка (GitHub Actions)
В репозитории настроен полностью автоматический рабочий процесс (CI/CD):
1. **Триггеры сборки**:
   - **Раз в неделю**: Каждое воскресенье в полночь по UTC GitHub Actions автоматически пересобирает драйвер, чтобы загрузить самые свежие ассеты с CDN производителя.
   - **При каждом коммите**: Автоматическая сборка запускается при любом `push` в ветки `main` или `master`.
   - **Вручную**: Сборку можно запустить в один клик в разделе **Actions** -> **"Build Offline Electron Driver"** -> **"Run workflow"**.
2. **Результат сборки**: После завершения процесса готовый ZIP-архив с портативной сборкой (`AJAZZ Local Driver.exe`) будет доступен для скачивания прямо в результатах запуска в разделе **Artifacts**!

---

## 🚀 Как запустить локально

### Требования
1. **Node.js** (версии 18 или выше)
2. **Python 3.x**

### 1. Пересборка офлайн-ассетов
Запустите скрипт для парсинга, скачивания динамических модулей и локального патчинга путей:
```bash
python build_offline.py
```

### 2. Запуск приложения
Запуск Electron в рабочем режиме:
```bash
npm install
npm start
```
*Для запуска с панелью разработчика (DevTools) для отладки:*
```bash
npm run start:debug
```

### 3. Сборка портативного `.exe`
Упаковка приложения в единую переносимую папку с файлом `AJAZZ Local Driver.exe` в директории `dist/`:
```bash
npm run build
```

---

> [!IMPORTANT]
> Для изменения раскладки, настроек Rapid Trigger и профилей подсветки клавиатура **должна быть обязательно подключена через USB-кабель** (не по Bluetooth и не по беспроводному адаптеру 2.4GHz). Передача команд по технологии WebHID поддерживается только по прямому проводу.
