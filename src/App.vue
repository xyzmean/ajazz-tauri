<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { 
  listDevices, 
  getDeviceInfo, 
  getGameMode, 
  setGameMode, 
  getLedEffect, 
  setLedEffect, 
  factoryReset,
  type DeviceSummary, 
  type DeviceInfo,
  type GameMode,
  type LedEffect
} from "./lib/api";

const devices = ref<DeviceSummary[]>([]);
const selected = ref<DeviceSummary | null>(null);
const info = ref<DeviceInfo | null>(null);
const gameMode = ref<GameMode | null>(null);
const ledEffect = ref<LedEffect | null>(null);

const error = ref<string | null>(null);
const successMessage = ref<string | null>(null);
const loading = ref(false);
const applying = ref(false);
const activeTab = ref("info");

const hex = (n: number) => "0x" + n.toString(16).padStart(4, "0");

// Computed properties to bridge primary and secondary RGB integers to HEX string color pickers
const primaryColor = computed({
  get() {
    if (!ledEffect.value) return "#3b82f6";
    const r = ledEffect.value.red.toString(16).padStart(2, "0");
    const g = ledEffect.value.green.toString(16).padStart(2, "0");
    const b = ledEffect.value.blue.toString(16).padStart(2, "0");
    return `#${r}${g}${b}`;
  },
  set(val: string) {
    if (!ledEffect.value) return;
    ledEffect.value.red = parseInt(val.slice(1, 3), 16);
    ledEffect.value.green = parseInt(val.slice(3, 5), 16);
    ledEffect.value.blue = parseInt(val.slice(5, 7), 16);
  }
});

const secondaryColor = computed({
  get() {
    if (!ledEffect.value) return "#10b981";
    const r = ledEffect.value.secondaryRed.toString(16).padStart(2, "0");
    const g = ledEffect.value.secondaryGreen.toString(16).padStart(2, "0");
    const b = ledEffect.value.secondaryBlue.toString(16).padStart(2, "0");
    return `#${r}${g}${b}`;
  },
  set(val: string) {
    if (!ledEffect.value) return;
    ledEffect.value.secondaryRed = parseInt(val.slice(1, 3), 16);
    ledEffect.value.secondaryGreen = parseInt(val.slice(3, 5), 16);
    ledEffect.value.secondaryBlue = parseInt(val.slice(5, 7), 16);
  }
});

async function refresh() {
  error.value = null;
  loading.value = true;
  try {
    devices.value = await listDevices();
    if (devices.value.length) {
      if (!selected.value) {
        await read(devices.value[0]);
      } else {
        // Re-read current selected
        const stillConnected = devices.value.find(d => d.path === selected.value?.path);
        if (stillConnected) {
          await read(stillConnected);
        } else {
          await read(devices.value[0]);
        }
      }
    } else {
      selected.value = null;
      info.value = null;
      gameMode.value = null;
      ledEffect.value = null;
    }
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function read(dev: DeviceSummary) {
  selected.value = dev;
  error.value = null;
  info.value = null;
  gameMode.value = null;
  ledEffect.value = null;
  loading.value = true;
  try {
    const devInfo = await getDeviceInfo(dev.path);
    info.value = devInfo;

    // Parallel fetch of Game Mode and LED Effect
    const [gm, led] = await Promise.all([
      getGameMode(dev.path, devInfo.frameVersion),
      getLedEffect(dev.path, devInfo.frameVersion)
    ]);
    gameMode.value = gm;
    ledEffect.value = led;
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function applyGameMode() {
  if (!selected.value || !gameMode.value || !info.value) return;
  error.value = null;
  applying.value = true;
  successMessage.value = null;
  try {
    await setGameMode(selected.value.path, gameMode.value, info.value.frameVersion);
    triggerSuccess("Настройки производительности успешно применены!");
  } catch (e) {
    error.value = String(e);
  } finally {
    applying.value = false;
  }
}

async function applyLedEffect() {
  if (!selected.value || !ledEffect.value) return;
  error.value = null;
  applying.value = true;
  successMessage.value = null;
  try {
    await setLedEffect(selected.value.path, ledEffect.value);
    triggerSuccess("Эффект подсветки успешно обновлен!");
  } catch (e) {
    error.value = String(e);
  } finally {
    applying.value = false;
  }
}

async function triggerReset(resetType: number) {
  if (!selected.value) return;
  let label = "полный сброс настроек";
  if (resetType === 1) label = "сброс клавиш";
  if (resetType === 2) label = "сброс подсветки";
  if (resetType === 5) label = "очистку калибровки";

  if (!confirm(`Вы действительно хотите выполнить ${label} клавиатуры?`)) return;

  error.value = null;
  applying.value = true;
  successMessage.value = null;
  try {
    await factoryReset(selected.value.path, resetType);
    triggerSuccess("Операция успешно завершена! Клавиатура перезагружена.");
    // Wait a brief moment and refresh
    setTimeout(refresh, 500);
  } catch (e) {
    error.value = String(e);
  } finally {
    applying.value = false;
  }
}

function triggerSuccess(msg: string) {
  successMessage.value = msg;
  setTimeout(() => {
    if (successMessage.value === msg) successMessage.value = null;
  }, 4000);
}

const lightModes = [
  { value: 0, label: "Выключена (Off)" },
  { value: 1, label: "Статичный (Static)" },
  { value: 2, label: "Одно касание (Single On)" },
  { value: 3, label: "Одно угасание (Single Off)" },
  { value: 4, label: "Звездное небо (Starry)" },
  { value: 5, label: "Мягкий снег (Snow)" },
  { value: 6, label: "Цветение (Bloom)" },
  { value: 7, label: "Дыхание (Breathing)" },
  { value: 8, label: "Спектр (Spectrum)" },
  { value: 9, label: "Фонтан (Fountain)" },
  { value: 10, label: "Перекресток (Colorful)" },
  { value: 11, label: "Волна (Wave)" },
  { value: 12, label: "Пик-Долина (Winding)" },
  { value: 13, label: "Триггер (Trigger)" },
  { value: 14, label: "Две птички (Two Birds)" },
  { value: 15, label: "Рябь (Ripple)" },
  { value: 16, label: "Поток (Flow)" },
  { value: 17, label: "Горный хребет (Mountain)" },
  { value: 18, label: "Дождевые капли (Rain)" },
  { value: 19, label: "Маятник (Shuttle)" }
];

onMounted(refresh);
</script>

<template>
  <div class="app">
    <header class="topbar">
      <div class="brand">
        <div class="logo-glow"></div>
        <h1>AJAZZ Driver Core</h1>
      </div>
      <button class="btn btn-secondary" :disabled="loading" @click="refresh">
        <svg v-if="loading" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="animation: spin 1s linear infinite;">
          <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
        </svg>
        <span>{{ loading ? "Обновление…" : "Сканировать устройства" }}</span>
      </button>
    </header>

    <main class="layout">
      <!-- Sidebar containing Device Selector -->
      <section class="sidebar">
        <h2>Устройства</h2>
        <p v-if="!devices.length && !loading" class="muted-hint" style="padding: 20px 0; text-align: left;">
          Клавиатуры не найдены. Подключите устройство по USB и выполните повторное сканирование.
        </p>
        <ul class="device-list">
          <li
            v-for="d in devices"
            :key="d.path"
            :class="{ active: selected?.path === d.path }"
            @click="read(d)"
          >
            <span class="name">{{ d.modelName ?? d.product ?? "HID Keyboard" }}</span>
            <span class="ids">{{ hex(d.vendorId) }}:{{ hex(d.productId) }}</span>
          </li>
        </ul>
      </section>

      <!-- Detail Panel containing Navigation Tabs -->
      <section class="detail">
        <div v-if="error" class="banner banner-error">{{ error }}</div>
        <div v-if="successMessage" class="banner banner-success">{{ successMessage }}</div>

        <template v-if="info">
          <!-- Active Device Header -->
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
            <div style="display: flex; flex-direction: column; gap: 4px;">
              <span style="font-size: 20px; font-weight: 700; color: #fff;">
                {{ selected?.modelName ?? selected?.product ?? "Устройство" }}
              </span>
              <span class="muted" style="font-size: 12px; font-family: ui-monospace, monospace;">
                HID: {{ selected?.path }}
              </span>
            </div>
            
            <!-- Sleek Battery Percentage Gauge -->
            <div style="display: flex; align-items: center; gap: 8px; background: rgba(255,255,255,0.03); padding: 6px 12px; border-radius: 20px; border: 1px solid var(--border);">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" :style="{ color: info.batteryLevel > 20 ? 'var(--success)' : 'var(--danger)' }">
                <rect x="2" y="7" width="16" height="10" rx="2" ry="2"/>
                <line x1="22" y1="11" x2="22" y2="13"/>
              </svg>
              <span style="font-weight: 700; color: #fff;">{{ info.batteryLevel }}%</span>
              <span v-if="info.chargeStatus === 1" class="muted" style="font-size: 11px; font-weight: 600; color: var(--success); text-transform: uppercase; letter-spacing: 0.05em;">Зарядка</span>
            </div>
          </div>

          <!-- Dynamic Tabs Selection Bar -->
          <nav class="tabs">
            <button class="tab-btn" :class="{ active: activeTab === 'info' }" @click="activeTab = 'info'">Свойства</button>
            <button class="tab-btn" :class="{ active: activeTab === 'lighting' }" @click="activeTab = 'lighting'" :disabled="!ledEffect">Подсветка</button>
            <button class="tab-btn" :class="{ active: activeTab === 'performance' }" @click="activeTab = 'performance'" :disabled="!gameMode">Производительность</button>
            <button class="tab-btn" :class="{ active: activeTab === 'maintenance' }" @click="activeTab = 'maintenance'">Обслуживание</button>
          </nav>

          <!-- Tab Content Sections -->
          <div class="card" style="flex: 1 1 auto; margin-top: 10px;">
            
            <!-- 1. Properties Tab -->
            <div v-if="activeTab === 'info'" class="grid">
              <div class="cell"><label>Модель</label><b>{{ selected?.modelName ?? "Универсальное ядро" }}</b></div>
              <div class="cell"><label>Версия прошивки</label><b>v{{ info.version }}</b></div>
              <div class="cell"><label>Vendor & Product ID</label><b>{{ hex(info.vendorId) }}:{{ hex(info.productId) }}</b></div>
              <div class="cell"><label>Профиль по умолчанию</label><b>Профиль {{ info.currentProfile }}</b></div>
              <div class="cell"><label>Ром-память (ROM)</label><b>{{ info.romSize }} КБ</b></div>
              <div class="cell"><label>Макро-пространство</label><b>{{ info.macroSpaceSize }} байт</b></div>
              <div class="cell"><label>Сенсорный тип</label><b>0x{{ info.sensor.toString(16).toUpperCase() }}</b></div>
              <div class="cell"><label>Рабочий интерфейс</label><b>{{ info.workMode === 1 ? "USB проводной" : info.workMode === 2 ? "2.4G Беспроводной" : "Bluetooth" }}</b></div>
              <div class="cell"><label>Версия фреймворка</label><b>v{{ info.frameVersion }}</b></div>
              <div class="cell"><label>Ядро подсветки</label><b>v{{ info.lightingVersion }}</b></div>
              <div class="cell"><label>Точность Rapid Trigger</label><b>{{ info.rtPrecision === 0 ? "1/100 (Стандарт)" : info.rtPrecision === 1 ? "1/1000 (Ультра)" : "1/1000 (Про)" }}</b></div>
            </div>

            <!-- 2. Lighting & LED Effects Tab -->
            <div v-else-if="activeTab === 'lighting' && ledEffect" style="display: flex; flex-direction: column; gap: 20px;">
              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));">
                
                <!-- LED Config Fields -->
                <div style="display: flex; flex-direction: column; gap: 14px;">
                  <div class="form-group">
                    <label>Режим подсветки</label>
                    <select v-model.number="ledEffect.mode" class="select-input">
                      <option v-for="mode in lightModes" :key="mode.value" :value="mode.value">
                        {{ mode.label }}
                      </option>
                    </select>
                  </div>

                  <div class="form-group">
                    <label>Яркость подсветки</label>
                    <div class="slider-container">
                      <input type="range" min="0" max="5" v-model.number="ledEffect.brightness" class="range-slider">
                      <span class="slider-value">{{ ledEffect.brightness * 20 }}%</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label>Скорость анимации</label>
                    <div class="slider-container">
                      <input type="range" min="1" max="5" v-model.number="ledEffect.speed" class="range-slider">
                      <span class="slider-value">{{ ledEffect.speed * 20 }}%</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label>Направление эффекта</label>
                    <select v-model.number="ledEffect.direction" class="select-input">
                      <option :value="0">Слева направо</option>
                      <option :value="1">Справа налево</option>
                      <option :value="2">Сверху вниз</option>
                      <option :value="3">Снизу вверх</option>
                    </select>
                  </div>
                </div>

                <!-- Custom Dual Color Pickers -->
                <div style="display: flex; flex-direction: column; gap: 20px; justify-content: center;">
                  <div class="color-picker-group">
                    <div class="color-picker-card">
                      <label>Основной цвет</label>
                      <div class="color-picker-wrapper" :style="{ backgroundColor: primaryColor }">
                        <input type="color" v-model="primaryColor" class="color-picker-native">
                      </div>
                      <span style="font-family: ui-monospace, monospace; font-size: 12px; color: var(--text-bright);">{{ primaryColor.toUpperCase() }}</span>
                    </div>

                    <div class="color-picker-card">
                      <label>Дополнительный</label>
                      <div class="color-picker-wrapper" :style="{ backgroundColor: secondaryColor }">
                        <input type="color" v-model="secondaryColor" class="color-picker-native">
                      </div>
                      <span style="font-family: ui-monospace, monospace; font-size: 12px; color: var(--text-bright);">{{ secondaryColor.toUpperCase() }}</span>
                    </div>
                  </div>

                  <!-- Color Glow Preview Block -->
                  <div style="height: 48px; border-radius: 8px; border: 1px solid var(--border); display: flex; align-items: center; justify-content: center; position: relative; overflow: hidden; background: #000;">
                    <div :style="{ position: 'absolute', top: 0, left: 0, bottom: 0, width: '50%', background: `linear-gradient(to right, ${primaryColor}, transparent)`, opacity: 0.4 }"></div>
                    <div :style="{ position: 'absolute', top: 0, right: 0, bottom: 0, width: '50%', background: `linear-gradient(to left, ${secondaryColor}, transparent)`, opacity: 0.4 }"></div>
                    <span style="font-size: 11px; font-weight: 700; color: #fff; z-index: 1; letter-spacing: 0.05em; text-transform: uppercase; text-shadow: 0 0 10px #000;">Световой профиль активен</span>
                  </div>
                </div>

              </div>

              <!-- Action Bar -->
              <div style="display: flex; justify-content: flex-end; border-top: 1px solid var(--border); padding-top: 16px; margin-top: 10px;">
                <button class="btn" :disabled="applying" @click="applyLedEffect">
                  <svg v-if="applying" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="animation: spin 1s linear infinite;">
                    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
                  </svg>
                  <span>{{ applying ? "Сохранение…" : "Применить подсветку" }}</span>
                </button>
              </div>
            </div>

            <!-- 3. Performance & Game Mode Tab -->
            <div v-else-if="activeTab === 'performance' && gameMode" style="display: flex; flex-direction: column; gap: 20px;">
              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));">
                
                <!-- Performance Settings -->
                <div style="display: flex; flex-direction: column; gap: 14px;">
                  <div class="form-group">
                    <label>Частота опроса (Report Rate)</label>
                    <select v-model.number="gameMode.reportRate" class="select-input">
                      <option :value="1">125 Гц (Энергосберегающий)</option>
                      <option :value="2">250 Гц (Базовый)</option>
                      <option :value="3">500 Гц (Баланс)</option>
                      <option :value="4">1000 Гц (Игровой)</option>
                    </select>
                  </div>

                  <div class="form-group">
                    <label>Режим ОС</label>
                    <select v-model.number="gameMode.systemMode" class="select-input">
                      <option :value="0">Windows (Стандартный)</option>
                      <option :value="1">macOS (Оптимизированный)</option>
                    </select>
                  </div>

                  <div class="form-group">
                    <label>Таймер сна (Sleep Time)</label>
                    <div class="slider-container">
                      <input type="range" min="1" max="60" v-model.number="gameMode.sleepTime" class="range-slider">
                      <span class="slider-value">{{ gameMode.sleepTime }} мин</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label>Задержка клавиш (Key Debounce)</label>
                    <div class="slider-container">
                      <input type="range" min="1" max="20" v-model.number="gameMode.keyDelay" class="range-slider">
                      <span class="slider-value">{{ gameMode.keyDelay }} мс</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label>Экран TFT дисплея (Display Time)</label>
                    <div class="slider-container">
                      <input type="range" min="5" max="60" v-model.number="gameMode.tftDisplayTime" class="range-slider">
                      <span class="slider-value">{{ gameMode.tftDisplayTime }} сек</span>
                    </div>
                  </div>
                </div>

                <!-- Toggle switches and RT Deadzones -->
                <div style="display: flex; flex-direction: column; gap: 12px;">
                  
                  <div class="switch-container">
                    <div class="switch-info">
                      <span class="switch-title">Пробуждение по одной клавише</span>
                      <span class="switch-desc">Экономит батарею, требуя полного нажатия для прогона.</span>
                    </div>
                    <label class="switch">
                      <input type="checkbox" v-model="gameMode.singleKeyWakeup" :true-value="1" :false-value="0">
                      <span class="slider"></span>
                    </label>
                  </div>

                  <div class="switch-container">
                    <div class="switch-info">
                      <span class="switch-title">Автоматическая калибровка магнитных осей</span>
                      <span class="switch-desc">Регулирует начальную точку магнитных переключателей.</span>
                    </div>
                    <label class="switch">
                      <input type="checkbox" v-model="gameMode.autoCalibration" :true-value="1" :false-value="0">
                      <span class="slider"></span>
                    </label>
                  </div>

                  <div class="switch-container">
                    <div class="switch-info">
                      <span class="switch-title">Быстрый запуск (FN Switch)</span>
                      <span class="switch-desc">Активация режима мгновенного отклика макросов.</span>
                    </div>
                    <label class="switch">
                      <input type="checkbox" v-model="gameMode.fnSwitch" :true-value="1" :false-value="0">
                      <span class="slider"></span>
                    </label>
                  </div>

                  <div class="form-group" style="margin-top: 10px;">
                    <label>Верхняя мертвая зона Rapid Trigger (Top Deadzone)</label>
                    <div class="slider-container">
                      <input type="range" min="0" max="0.5" step="0.01" v-model.number="gameMode.topDeadZone" class="range-slider">
                      <span class="slider-value">{{ (gameMode.topDeadZone * 100).toFixed(0) }}%</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label>Нижняя мертвая зона Rapid Trigger (Bottom Deadzone)</label>
                    <div class="slider-container">
                      <input type="range" min="0" max="0.5" step="0.01" v-model.number="gameMode.bottomDeadZone" class="range-slider">
                      <span class="slider-value">{{ (gameMode.bottomDeadZone * 100).toFixed(0) }}%</span>
                    </div>
                  </div>
                </div>

              </div>

              <!-- Action Bar -->
              <div style="display: flex; justify-content: flex-end; border-top: 1px solid var(--border); padding-top: 16px; margin-top: 10px;">
                <button class="btn" :disabled="applying" @click="applyGameMode">
                  <svg v-if="applying" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="animation: spin 1s linear infinite;">
                    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
                  </svg>
                  <span>{{ applying ? "Запись…" : "Сохранить производительность" }}</span>
                </button>
              </div>
            </div>

            <!-- 4. Maintenance (Reset / Calibration) Tab -->
            <div v-else-if="activeTab === 'maintenance'" style="display: flex; flex-direction: column; gap: 20px;">
              <span style="font-size: 14px; font-weight: 500; color: var(--muted); margin-bottom: 10px; display: block;">
                Панель аварийного обслуживания и калибровки клавиатуры. Выберите нужное действие:
              </span>

              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));">
                
                <!-- Reset All -->
                <div class="cell" style="justify-content: space-between; border-color: rgba(239, 68, 68, 0.25);">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 700; color: #fff; font-size: 14px;">Полный заводской сброс</span>
                    <span class="muted" style="font-size: 11px;">Сброс абсолютно всех раскладок, эффектов и очистка сохраненных макро-файлов.</span>
                  </div>
                  <button class="btn btn-danger" style="width: 100%; justify-content: center;" :disabled="applying" @click="triggerReset(255)">
                    Сбросить всё
                  </button>
                </div>

                <!-- Clear Calibration -->
                <div class="cell" style="justify-content: space-between; border-color: rgba(245, 158, 11, 0.25);">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 700; color: #fff; font-size: 14px;">Очистка калибровки</span>
                    <span class="muted" style="font-size: 11px;">Очищает сохраненные данные калибровки датчиков Холла для магнитных клавиш.</span>
                  </div>
                  <button class="btn btn-secondary" style="width: 100%; justify-content: center; border-color: var(--warning); color: var(--warning);" :disabled="applying" @click="triggerReset(5)">
                    Сбросить калибровку
                  </button>
                </div>

                <!-- Reset Lighting -->
                <div class="cell" style="justify-content: space-between;">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 700; color: #fff; font-size: 14px;">Сброс световых профилей</span>
                    <span class="muted" style="font-size: 11px;">Восстановление дефолтных анимаций и сброс цветовых матриц на заводские значения.</span>
                  </div>
                  <button class="btn btn-secondary" style="width: 100%; justify-content: center;" :disabled="applying" @click="triggerReset(2)">
                    Сбросить подсветку
                  </button>
                </div>

                <!-- Reset Keys -->
                <div class="cell" style="justify-content: space-between;">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 700; color: #fff; font-size: 14px;">Сброс раскладки</span>
                    <span class="muted" style="font-size: 11px;">Возврат стандартного биндинга клавиш и сброс кастомного профиля Fn-слоя.</span>
                  </div>
                  <button class="btn btn-secondary" style="width: 100%; justify-content: center;" :disabled="applying" @click="triggerReset(1)">
                    Сбросить клавиши
                  </button>
                </div>

              </div>
            </div>

          </div>
        </template>

        <!-- No Keyboard Selected Splash -->
        <div v-else-if="!error" class="muted-hint">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="margin-bottom: 16px; opacity: 0.6; color: var(--accent);">
            <rect x="2" y="4" width="20" height="16" rx="2" ry="2"/>
            <path d="M6 8h.01M10 8h.01M14 8h.01M18 8h.01M6 12h.01M10 12h.01M14 12h.01M18 12h.01M7 16h10"/>
          </svg>
          <p style="font-size: 15px; font-weight: 500; color: var(--text-bright); margin: 0 0 4px;">Драйвер ядра не подключен</p>
          <p style="font-size: 13px; margin: 0;">Выберите Ajazz-клавиатуру в списке слева или обновите список устройств, чтобы начать управление.</p>
        </div>
      </section>
    </main>
  </div>
</template>

<style>
/* Local spin animation utility */
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
