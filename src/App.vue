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
  streamLedFrame,
  sendMusicData,
  type DeviceSummary, 
  type DeviceInfo,
  type GameMode,
  type LedEffect,
  type LedColor
} from "./lib/api";

// --- Hardware Key Backlight Layout Matrix ---
const keysList = [
  // Row 1 (Y=0)
  { label: "Esc", idx: 0, x: 0, y: 0, w: 1 },
  { label: "F1", idx: 1, x: 2, y: 0, w: 1 },
  { label: "F2", idx: 2, x: 3, y: 0, w: 1 },
  { label: "F3", idx: 3, x: 4, y: 0, w: 1 },
  { label: "F4", idx: 4, x: 5, y: 0, w: 1 },
  { label: "F5", idx: 5, x: 6.5, y: 0, w: 1 },
  { label: "F6", idx: 6, x: 7.5, y: 0, w: 1 },
  { label: "F7", idx: 7, x: 8.5, y: 0, w: 1 },
  { label: "F8", idx: 8, x: 9.5, y: 0, w: 1 },
  { label: "F9", idx: 9, x: 11, y: 0, w: 1 },
  { label: "F10", idx: 10, x: 12, y: 0, w: 1 },
  { label: "F11", idx: 11, x: 13, y: 0, w: 1 },
  { label: "F12", idx: 12, x: 14, y: 0, w: 1 },
  { label: "Del", idx: 13, x: 15.5, y: 0, w: 1 },
  { label: "Ins", idx: 14, x: 16.5, y: 0, w: 1 },
  { label: "PgUp", idx: 15, x: 17.5, y: 0, w: 1 },
  { label: "PgDn", idx: 16, x: 18.5, y: 0, w: 1 },

  // Row 2 (Y=1)
  { label: "~", idx: 17, x: 0, y: 1, w: 1 },
  { label: "1", idx: 18, x: 1, y: 1, w: 1 },
  { label: "2", idx: 19, x: 2, y: 1, w: 1 },
  { label: "3", idx: 20, x: 3, y: 1, w: 1 },
  { label: "4", idx: 21, x: 4, y: 1, w: 1 },
  { label: "5", idx: 22, x: 5, y: 1, w: 1 },
  { label: "6", idx: 23, x: 6, y: 1, w: 1 },
  { label: "7", idx: 24, x: 7, y: 1, w: 1 },
  { label: "8", idx: 25, x: 8, y: 1, w: 1 },
  { label: "9", idx: 26, x: 9, y: 1, w: 1 },
  { label: "0", idx: 27, x: 10, y: 1, w: 1 },
  { label: "-", idx: 28, x: 11, y: 1, w: 1 },
  { label: "=", idx: 29, x: 12, y: 1, w: 1 },
  { label: "Backspace", idx: 30, x: 13, y: 1, w: 2 },
  { label: "NumL", idx: 31, x: 15.5, y: 1, w: 1 },
  { label: "/", idx: 32, x: 16.5, y: 1, w: 1 },
  { label: "*", idx: 33, x: 17.5, y: 1, w: 1 },
  { label: "-", idx: 34, x: 18.5, y: 1, w: 1 },

  // Row 3 (Y=2)
  { label: "Tab", idx: 35, x: 0, y: 2, w: 1.5 },
  { label: "Q", idx: 36, x: 1.5, y: 2, w: 1 },
  { label: "W", idx: 37, x: 2.5, y: 2, w: 1 },
  { label: "E", idx: 38, x: 3.5, y: 2, w: 1 },
  { label: "R", idx: 39, x: 4.5, y: 2, w: 1 },
  { label: "T", idx: 40, x: 5.5, y: 2, w: 1 },
  { label: "Y", idx: 41, x: 6.5, y: 2, w: 1 },
  { label: "U", idx: 42, x: 7.5, y: 2, w: 1 },
  { label: "I", idx: 43, x: 8.5, y: 2, w: 1 },
  { label: "O", idx: 44, x: 9.5, y: 2, w: 1 },
  { label: "P", idx: 45, x: 10.5, y: 2, w: 1 },
  { label: "[", idx: 46, x: 11.5, y: 2, w: 1 },
  { label: "]", idx: 47, x: 12.5, y: 2, w: 1 },
  { label: "\\", idx: 48, x: 13.5, y: 2, w: 1.5 },
  { label: "7", idx: 49, x: 15.5, y: 2, w: 1 },
  { label: "8", idx: 50, x: 16.5, y: 2, w: 1 },
  { label: "9", idx: 51, x: 17.5, y: 2, w: 1 },
  { label: "+", idx: 52, x: 18.5, y: 2, w: 1, h: 2 },

  // Row 4 (Y=3)
  { label: "Caps", idx: 53, x: 0, y: 3, w: 1.75 },
  { label: "A", idx: 54, x: 1.75, y: 3, w: 1 },
  { label: "S", idx: 55, x: 2.75, y: 3, w: 1 },
  { label: "D", idx: 56, x: 3.75, y: 3, w: 1 },
  { label: "F", idx: 57, x: 4.75, y: 3, w: 1 },
  { label: "G", idx: 58, x: 5.75, y: 3, w: 1 },
  { label: "H", idx: 59, x: 6.75, y: 3, w: 1 },
  { label: "J", idx: 60, x: 7.75, y: 3, w: 1 },
  { label: "K", idx: 61, x: 8.75, y: 3, w: 1 },
  { label: "L", idx: 62, x: 9.75, y: 3, w: 1 },
  { label: ";", idx: 63, x: 10.75, y: 3, w: 1 },
  { label: "'", idx: 64, x: 11.75, y: 3, w: 1 },
  { label: "Enter", idx: 65, x: 12.75, y: 3, w: 2.25 },
  { label: "4", idx: 66, x: 15.5, y: 3, w: 1 },
  { label: "5", idx: 67, x: 16.5, y: 3, w: 1 },
  { label: "6", idx: 68, x: 17.5, y: 3, w: 1 },

  // Row 5 (Y=4)
  { label: "Shift", idx: 69, x: 0, y: 4, w: 2.25 },
  { label: "Z", idx: 70, x: 2.25, y: 4, w: 1 },
  { label: "X", idx: 71, x: 3.25, y: 4, w: 1 },
  { label: "C", idx: 72, x: 4.25, y: 4, w: 1 },
  { label: "V", idx: 73, x: 5.25, y: 4, w: 1 },
  { label: "B", idx: 74, x: 6.25, y: 4, w: 1 },
  { label: "N", idx: 75, x: 7.25, y: 4, w: 1 },
  { label: "M", idx: 76, x: 8.25, y: 4, w: 1 },
  { label: ",", idx: 77, x: 9.25, y: 4, w: 1 },
  { label: ".", idx: 78, x: 10.25, y: 4, w: 1 },
  { label: "/", idx: 79, x: 11.25, y: 4, w: 1 },
  { label: "Shift", idx: 80, x: 12.25, y: 4, w: 1.75 },
  { label: "▲", idx: 81, x: 14.25, y: 4, w: 1 },
  { label: "1", idx: 82, x: 15.5, y: 4, w: 1 },
  { label: "2", idx: 83, x: 16.5, y: 4, w: 1 },
  { label: "3", idx: 84, x: 17.5, y: 4, w: 1 },
  { label: "Enter", idx: 85, x: 18.5, y: 4, w: 1, h: 2 },

  // Row 6 (Y=5)
  { label: "Ctrl", idx: 86, x: 0, y: 5, w: 1.25 },
  { label: "Win", idx: 87, x: 1.25, y: 5, w: 1.25 },
  { label: "Alt", idx: 88, x: 2.5, y: 5, w: 1.25 },
  { label: "Space", idx: 89, x: 3.75, y: 5, w: 6.25 },
  { label: "Alt", idx: 90, x: 10, y: 5, w: 1.25 },
  { label: "Fn", idx: 91, x: 11.25, y: 5, w: 1.25 },
  { label: "Ctrl", idx: 92, x: 12.5, y: 5, w: 1.25 },
  { label: "◀", idx: 93, x: 14.25, y: 5, w: 1 },
  { label: "▼", idx: 94, x: 15.25, y: 5, w: 1 },
  { label: "▶", idx: 95, x: 16.25, y: 5, w: 1 },
  { label: "0", idx: 96, x: 17.5, y: 5, w: 2 },
  { label: ".", idx: 97, x: 18.5, y: 5, w: 1 }
];

const keyboardRows = computed(() => {
  const rows: any[][] = Array.from({ length: 6 }, () => []);
  for (const k of keysList) {
    rows[k.y].push(k);
  }
  return rows;
});

// --- High-Performance LZW GIF Decoder ---
function decompressLZW(minCodeSize: number, subBlocks: Uint8Array[], pixelCount: number): Uint8Array {
  const pixels = new Uint8Array(pixelCount);
  const clearCode = 1 << minCodeSize;
  const endCode = clearCode + 1;
  let codeSize = minCodeSize + 1;
  let codeMask = (1 << codeSize) - 1;
  
  let totalLength = 0;
  for (const b of subBlocks) totalLength += b.length;
  const data = new Uint8Array(totalLength);
  let dpos = 0;
  for (const b of subBlocks) {
    data.set(b, dpos);
    dpos += b.length;
  }
  
  let bitBuf = 0;
  let bitCount = 0;
  let dataPos = 0;
  
  function getCode(): number {
    while (bitCount < codeSize) {
      if (dataPos >= data.length) return endCode;
      bitBuf |= data[dataPos++] << bitCount;
      bitCount += 8;
    }
    const code = bitBuf & codeMask;
    bitBuf >>>= codeSize;
    bitCount -= codeSize;
    return code;
  }
  
  const prefix = new Int32Array(4096);
  const suffix = new Uint8Array(4096);
  const stack = new Uint8Array(4097);
  let stackPtr = 0;
  
  let dictSize = clearCode + 2;
  
  function resetDict() {
    codeSize = minCodeSize + 1;
    codeMask = (1 << codeSize) - 1;
    dictSize = clearCode + 2;
    for (let i = 0; i < clearCode; i++) {
      prefix[i] = -1;
      suffix[i] = i;
    }
  }
  
  resetDict();
  
  let oldCode = -1;
  let pixelPos = 0;
  
  while (pixelPos < pixelCount) {
    const code = getCode();
    if (code === clearCode) {
      resetDict();
      oldCode = -1;
      continue;
    }
    if (code === endCode) break;
    
    if (oldCode === -1) {
      pixels[pixelPos++] = suffix[code];
      oldCode = code;
      continue;
    }
    
    let currentCode = code;
    if (code >= dictSize) {
      stack[stackPtr++] = suffix[oldCode];
      currentCode = oldCode;
    }
    
    while (currentCode >= clearCode) {
      stack[stackPtr++] = suffix[currentCode];
      currentCode = prefix[currentCode];
    }
    
    const firstPixel = suffix[currentCode];
    pixels[pixelPos++] = firstPixel;
    
    while (stackPtr > 0) {
      pixels[pixelPos++] = stack[--stackPtr];
    }
    
    if (code >= dictSize) {
      pixels[pixelPos - 1] = firstPixel;
    }
    
    if (dictSize < 4096) {
      prefix[dictSize] = oldCode;
      suffix[dictSize] = firstPixel;
      dictSize++;
      if (dictSize === codeMask + 1 && codeSize < 12) {
        codeSize++;
        codeMask = (1 << codeSize) - 1;
      }
    }
    oldCode = code;
  }
  
  return pixels;
}

async function parseGIF(arrayBuffer: ArrayBuffer) {
  const u8 = new Uint8Array(arrayBuffer);
  let pos = 0;
  
  const readByte = () => u8[pos++];
  const readUInt16 = () => {
    const b1 = u8[pos++];
    const b2 = u8[pos++];
    return b1 | (b2 << 8);
  };
  
  const sig = String.fromCharCode(...u8.subarray(0, 6));
  if (sig !== "GIF87a" && sig !== "GIF89a") {
    throw new Error("Invalid GIF signature");
  }
  pos = 6;
  
  const width = readUInt16();
  const height = readUInt16();
  const packed = readByte();
  const hasGlobalColorTable = (packed & 0x80) !== 0;
  const globalColorTableSize = 1 << ((packed & 0x07) + 1);
  const bgColorIndex = readByte();
  const pixelAspectRatio = readByte();
  
  let globalColorTable: Uint8Array | null = null;
  if (hasGlobalColorTable) {
    globalColorTable = u8.subarray(pos, pos + globalColorTableSize * 3);
    pos += globalColorTableSize * 3;
  }
  
  const frames: { width: number; height: number; delay: number; pixels: Uint8Array }[] = [];
  let gceDelay = 10;
  let gceTransparentColorIndex = -1;
  let gceHasTransparency = false;
  
  while (pos < u8.length) {
    const blockType = readByte();
    if (blockType === 0x3B) break;
    
    if (blockType === 0x21) {
      const extType = readByte();
      if (extType === 0xF9) {
        const blockSize = readByte();
        const gcePacked = readByte();
        gceDelay = readUInt16();
        if (gceDelay < 2) gceDelay = 10;
        gceTransparentColorIndex = readByte();
        gceHasTransparency = (gcePacked & 0x01) !== 0;
        readByte();
      } else {
        let blockSize = readByte();
        while (blockSize > 0) {
          pos += blockSize;
          blockSize = readByte();
        }
      }
    } else if (blockType === 0x2C) {
      const left = readUInt16();
      const top = readUInt16();
      const w = readUInt16();
      const h = readUInt16();
      const imgPacked = readByte();
      const hasLocalColorTable = (imgPacked & 0x80) !== 0;
      const localColorTableSize = 1 << ((imgPacked & 0x07) + 1);
      
      let localColorTable: Uint8Array | null = null;
      if (hasLocalColorTable) {
        localColorTable = u8.subarray(pos, pos + localColorTableSize * 3);
        pos += localColorTableSize * 3;
      }
      
      const minCodeSize = readByte();
      const subBlocks: Uint8Array[] = [];
      let blockSize = readByte();
      while (blockSize > 0) {
        subBlocks.push(u8.subarray(pos, pos + blockSize));
        pos += blockSize;
        blockSize = readByte();
      }
      
      const pixelCount = w * h;
      const indexPixels = decompressLZW(minCodeSize, subBlocks, pixelCount);
      const palette = localColorTable || globalColorTable || new Uint8Array(768);
      
      const rgbaPixels = new Uint8Array(w * h * 4);
      for (let i = 0; i < pixelCount; i++) {
        const idx = indexPixels[i];
        if (gceHasTransparency && idx === gceTransparentColorIndex) {
          rgbaPixels[i * 4] = 0;
          rgbaPixels[i * 4 + 1] = 0;
          rgbaPixels[i * 4 + 2] = 0;
          rgbaPixels[i * 4 + 3] = 0;
        } else {
          rgbaPixels[i * 4] = palette[idx * 3];
          rgbaPixels[i * 4 + 1] = palette[idx * 3 + 1];
          rgbaPixels[i * 4 + 2] = palette[idx * 3 + 2];
          rgbaPixels[i * 4 + 3] = 255;
        }
      }
      
      frames.push({
        width: w,
        height: h,
        delay: gceDelay * 10,
        pixels: rgbaPixels
      });
    } else {
      break;
    }
  }
  
  return frames;
}

// --- Vue Reactive States ---
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

// Interactive Virtual LED backlighting states
const liveKeyColors = ref<Record<number, string>>({});

// Ambient Sync variables
const isAmbientActive = ref(false);
let ambientInterval: any = null;
let ambientMediaStream: MediaStream | null = null;
let ambientVideo: HTMLVideoElement | null = null;
let ambientCanvas: HTMLCanvasElement | null = null;

// Equalizer variables
const isEqActive = ref(false);
const eqMode = ref("custom"); // "custom" or "native"
const eqTheme = ref(0);
const eqSpeed = ref(3);
const eqBrightness = ref(4);
let eqInterval: any = null;
let eqAudioContext: AudioContext | null = null;
let eqAnalyser: AnalyserNode | null = null;
let eqMediaStream: MediaStream | null = null;
const eqBarsData = ref<number[]>(Array.from({ length: 21 }, () => 0));

// GIF variables
const isGifPlaying = ref(false);
const gifFrames = ref<any[]>([]);
const activeGifFrameIndex = ref(0);
const uploadedGifName = ref<string | null>(null);
const fileInput = ref<HTMLInputElement | null>(null);
let gifTimeout: any = null;

const hex = (n: number) => "0x" + n.toString(16).padStart(4, "0");

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
    updateStaticBacklight();
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
    updateStaticBacklight();
  }
});

// Update standard visual preview keys when lighting values change
function updateStaticBacklight() {
  if (!ledEffect.value) return;
  const col = primaryColor.value;
  for (const k of keysList) {
    liveKeyColors.value[k.idx] = col;
  }
}

// Switch tabs while cleanly terminating any active background streaming loop
function switchTab(tab: string) {
  stopAmbientSync();
  stopEqualizer();
  stopGifPlayback();
  activeTab.value = tab;
  
  if (tab === 'lighting') {
    updateStaticBacklight();
  } else {
    // Reset preview to default dark purple state for custom streaming visuals
    for (const k of keysList) {
      liveKeyColors.value[k.idx] = "rgba(255,255,255,0.02)";
    }
  }
}

// --- Active Backlight Streaming Bridge with On-Screen Preview Mirror ---
async function streamBacklight(frame: LedColor[]) {
  if (!selected.value) return;
  
  // 1. Instantly update visual keyboard preview
  for (const k of frame) {
    liveKeyColors.value[k.idx] = `rgb(${k.r}, ${k.g}, ${k.b})`;
  }
  
  // 2. Stream raw packet bytes down to Rust hidapi
  try {
    await streamLedFrame(selected.value.path, frame);
  } catch (e) {
    console.error("Backlight packet streaming failure:", e);
  }
}

async function refresh() {
  error.value = null;
  loading.value = true;
  try {
    devices.value = await listDevices();
    if (devices.value.length) {
      if (!selected.value) {
        await read(devices.value[0]);
      } else {
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
  // Clear any active screen or music loops before switching active devices
  stopAmbientSync();
  stopEqualizer();
  stopGifPlayback();

  selected.value = dev;
  error.value = null;
  info.value = null;
  gameMode.value = null;
  ledEffect.value = null;
  loading.value = true;
  try {
    const devInfo = await getDeviceInfo(dev.path);
    info.value = devInfo;

    const [gm, led] = await Promise.all([
      getGameMode(dev.path, devInfo.frameVersion),
      getLedEffect(dev.path, devInfo.frameVersion)
    ]);
    gameMode.value = gm;
    ledEffect.value = led;
    
    // Set initial static key preview
    updateStaticBacklight();
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
    triggerSuccess("Настройки производительности применены!");
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
    triggerSuccess("Эффект стандартной подсветки сохранен!");
  } catch (e) {
    error.value = String(e);
  } finally {
    applying.value = false;
  }
}

async function triggerReset(resetType: number) {
  if (!selected.value) return;
  let label = "полный заводской сброс";
  if (resetType === 1) label = "сброс клавиш";
  if (resetType === 2) label = "сброс подсветки";
  if (resetType === 5) label = "очистку калибровки";

  if (!confirm(`Вы действительно хотите выполнить ${label} клавиатуры?`)) return;

  error.value = null;
  applying.value = true;
  successMessage.value = null;
  try {
    stopAmbientSync();
    stopEqualizer();
    stopGifPlayback();
    
    await factoryReset(selected.value.path, resetType);
    triggerSuccess("Клавиатура успешно сброшена!");
    setTimeout(refresh, 800);
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

// --- 1. Screen Mirroring / Ambient Light Sync Logic ---
async function startAmbientSync() {
  if (!selected.value) return;
  error.value = null;
  try {
    // Capture user screen or custom window
    ambientMediaStream = await navigator.mediaDevices.getDisplayMedia({
      video: { width: 1280, height: 720, frameRate: 30 }
    });
    
    ambientVideo = document.createElement("video");
    ambientVideo.srcObject = ambientMediaStream;
    ambientVideo.autoplay = true;
    ambientVideo.playsInline = true;
    
    ambientCanvas = document.createElement("canvas");
    ambientCanvas.width = 64;
    ambientCanvas.height = 36;
    const ctx = ambientCanvas.getContext("2d");
    if (!ctx) throw new Error("Could not initialize canvas graphics context");
    
    isAmbientActive.value = true;
    
    ambientInterval = setInterval(async () => {
      if (!ambientVideo || ambientVideo.readyState < ambientVideo.HAVE_CURRENT_DATA) return;
      ctx.drawImage(ambientVideo, 0, 0, ambientCanvas!.width, ambientCanvas!.height);
      const imgData = ctx.getImageData(0, 0, ambientCanvas!.width, ambientCanvas!.height);
      const data = imgData.data;
      
      const frame: LedColor[] = [];
      for (const key of keysList) {
        // Map 2D physical key matrix space to downsampled coordinates
        const px = Math.min(63, Math.round((key.x / 19.5) * 63));
        const py = Math.min(35, Math.round((key.y / 5) * 35));
        
        const offset = (py * 64 + px) * 4;
        const r = data[offset];
        const g = data[offset + 1];
        const b = data[offset + 2];
        
        frame.push({ idx: key.idx, r, g, b });
      }
      await streamBacklight(frame);
    }, 40); // 25 FPS
    
    ambientMediaStream.getVideoTracks()[0].onended = () => {
      stopAmbientSync();
    };
    
    triggerSuccess("Эмбиент-зеркалирование экрана успешно запущено!");
  } catch (e) {
    error.value = "Ошибка инициализации зеркалирования: " + String(e);
    stopAmbientSync();
  }
}

function stopAmbientSync() {
  isAmbientActive.value = false;
  if (ambientInterval) {
    clearInterval(ambientInterval);
    ambientInterval = null;
  }
  if (ambientMediaStream) {
    ambientMediaStream.getTracks().forEach(t => t.stop());
    ambientMediaStream = null;
  }
  ambientVideo = null;
  ambientCanvas = null;
}

// --- 2. Cyber Loopback Audio Equalizer Logic ---
async function startEqualizer() {
  if (!selected.value) return;
  error.value = null;
  try {
    // Capture user system loopback sound card or mic input
    eqMediaStream = await navigator.mediaDevices.getUserMedia({ audio: true, video: false });
    eqAudioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    const source = eqAudioContext.createMediaStreamSource(eqMediaStream);
    eqAnalyser = eqAudioContext.createAnalyser();
    eqAnalyser.fftSize = 64; // Slices spectrum into 32 frequency ranges
    source.connect(eqAnalyser);
    
    const bufferLength = eqAnalyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);
    
    isEqActive.value = true;
    
    eqInterval = setInterval(async () => {
      if (!eqAnalyser || !selected.value) return;
      eqAnalyser.getByteFrequencyData(dataArray);
      
      // Re-map 32 ranges to 21 standard equalizers
      const amplitudes: number[] = [];
      for (let i = 0; i < 21; i++) {
        const idx = Math.floor((i / 21) * bufferLength);
        amplitudes.push(dataArray[idx]);
      }
      
      // Update bars on UI screen
      eqBarsData.value = amplitudes.map(v => Math.round((v / 255) * 100));
      
      if (eqMode.value === "native") {
        try {
          await sendMusicData(
            selected.value.path,
            eqTheme.value,
            eqSpeed.value,
            eqBrightness.value,
            amplitudes
          );
        } catch (e) {
          console.error("Native spectrum call fail:", e);
        }
      } else {
        // Project real-time custom sound bouncing columns onto physical backlight matrix!
        const frame: LedColor[] = [];
        for (const key of keysList) {
          const colIndex = Math.min(20, Math.floor((key.x / 19.5) * 20));
          const amp = amplitudes[colIndex] || 0;
          
          const heightRatio = amp / 255;
          const keyHeightRatio = (5 - key.y) / 5;
          
          if (keyHeightRatio <= heightRatio) {
            // Neon pink-to-cyan audio column gradient
            const ratio = (5 - key.y) / 5;
            const r = Math.round(255 * (1 - ratio) + 0 * ratio);
            const g = Math.round(0 * (1 - ratio) + 240 * ratio);
            const b = Math.round(127 * (1 - ratio) + 255 * ratio);
            frame.push({ idx: key.idx, r, g, b });
          } else {
            // Background deep violet ambient atmosphere
            frame.push({ idx: key.idx, r: 12, g: 8, b: 24 });
          }
        }
        await streamBacklight(frame);
      }
    }, 40); // 25 FPS
    
    triggerSuccess("Музыкальный визуализатор запущен!");
  } catch (e) {
    error.value = "Ошибка запуска микрофона/аудио: " + String(e);
    stopEqualizer();
  }
}

function stopEqualizer() {
  isEqActive.value = false;
  if (eqInterval) {
    clearInterval(eqInterval);
    eqInterval = null;
  }
  if (eqAudioContext) {
    eqAudioContext.close();
    eqAudioContext = null;
  }
  if (eqMediaStream) {
    eqMediaStream.getTracks().forEach(t => t.stop());
    eqMediaStream = null;
  }
  eqAnalyser = null;
  eqBarsData.value = Array.from({ length: 21 }, () => 0);
}

// --- 3. Retro GIF Backlight Pixel-Art Logic ---
async function handleGifUpload(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  await loadGifFile(file);
}

async function handleGifDrop(event: DragEvent) {
  event.preventDefault();
  const file = event.dataTransfer?.files?.[0];
  if (file && file.name.endsWith(".gif")) {
    await loadGifFile(file);
  } else {
    error.value = "Ошибка: перетащите файл расширения .gif";
  }
}

async function loadGifFile(file: File) {
  error.value = null;
  uploadedGifName.value = file.name;
  stopGifPlayback();
  
  try {
    const reader = new FileReader();
    reader.onload = async (e) => {
      try {
        const buffer = e.target?.result as ArrayBuffer;
        gifFrames.value = await parseGIF(buffer);
        if (gifFrames.value.length === 0) {
          throw new Error("Не найдено кадров в анимированном GIF-файле");
        }
        triggerSuccess(`Файл ${file.name} разобран! Кадров: ${gifFrames.value.length}`);
        startGifPlayback();
      } catch (err) {
        error.value = "Ошибка декодирования структуры GIF: " + String(err);
      }
    };
    reader.readAsArrayBuffer(file);
  } catch (err) {
    error.value = "Ошибка чтения буфера файла: " + String(err);
  }
}

function startGifPlayback() {
  if (gifFrames.value.length === 0 || !selected.value) return;
  isGifPlaying.value = true;
  activeGifFrameIndex.value = 0;
  playNextGifFrame();
}

async function playNextGifFrame() {
  if (!isGifPlaying.value || gifFrames.value.length === 0) return;
  
  const frameData = gifFrames.value[activeGifFrameIndex.value];
  const w = frameData.width;
  const h = frameData.height;
  const pixels = frameData.pixels;
  const frame: LedColor[] = [];
  
  const AR_kb = 416 / 148; // Physical aspect ratio of keyboard (2.81)
  const AR_gif = w / h;
  
  for (const key of keysList) {
    const u = key.x / 19.5;
    const v = key.y / 5;
    
    let u_mapped, v_mapped, in_bounds;
    if (AR_gif > AR_kb) {
      const scale = AR_gif / AR_kb;
      u_mapped = u;
      v_mapped = (v - 0.5) * scale + 0.5;
      in_bounds = v_mapped >= 0 && v_mapped <= 1;
    } else {
      const scale = AR_kb / AR_gif;
      u_mapped = (u - 0.5) * scale + 0.5;
      v_mapped = v;
      in_bounds = u_mapped >= 0 && u_mapped <= 1;
    }
    
    if (in_bounds) {
      const px = Math.min(w - 1, Math.max(0, Math.round(u_mapped * (w - 1))));
      const py = Math.min(h - 1, Math.max(0, Math.round(v_mapped * (h - 1))));
      
      const offset = (py * w + px) * 4;
      const r = pixels[offset];
      const g = pixels[offset + 1];
      const b = pixels[offset + 2];
      
      frame.push({ idx: key.idx, r, g, b });
    } else {
      // Ambient dark background padding
      frame.push({ idx: key.idx, r: 12, g: 8, b: 24 });
    }
  }
  
  await streamBacklight(frame);
  
  activeGifFrameIndex.value = (activeGifFrameIndex.value + 1) % gifFrames.value.length;
  gifTimeout = setTimeout(playNextGifFrame, frameData.delay);
}

function stopGifPlayback() {
  isGifPlaying.value = false;
  if (gifTimeout) {
    clearTimeout(gifTimeout);
    gifTimeout = null;
  }
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
        <div class="logo-glow" style="background: var(--neon-pink); box-shadow: 0 0 10px var(--neon-pink), 0 0 20px var(--neon-pink);"></div>
        <h1 style="background: linear-gradient(to right, var(--neon-pink), var(--neon-cyan)); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">AJAZZ CYBER COMPANION</h1>
      </div>
      <button class="btn btn-secondary" :disabled="loading" @click="refresh" style="border-color: rgba(0, 240, 255, 0.4); box-shadow: 0 0 8px rgba(0,240,255,0.1);">
        <svg v-if="loading" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="animation: spin 1s linear infinite; color: var(--neon-cyan);">
          <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
        </svg>
        <span style="color: var(--neon-cyan); font-weight: 700;">{{ loading ? "Обновление…" : "Сканировать" }}</span>
      </button>
    </header>

    <main class="layout">
      <!-- Sidebar Selector -->
      <section class="sidebar" style="background: rgba(8, 10, 18, 0.7); backdrop-filter: blur(12px);">
        <h2>Подключенные устройства</h2>
        <p v-if="!devices.length && !loading" class="muted-hint" style="padding: 20px 0; text-align: left; font-size: 12px;">
          Устройства не обнаружены. Пожалуйста, подключите клавиатуру к USB-порту.
        </p>
        <ul class="device-list">
          <li
            v-for="d in devices"
            :key="d.path"
            :class="{ active: selected?.path === d.path }"
            @click="read(d)"
            style="border-color: rgba(255,255,255,0.06); background: rgba(255,255,255,0.02);"
          >
            <span class="name" style="font-size: 13px; font-weight: 700;">{{ d.modelName ?? d.product ?? "HID-Клавиатура" }}</span>
            <span class="ids" style="color: var(--neon-cyan);">{{ hex(d.vendorId) }}:{{ hex(d.productId) }}</span>
          </li>
        </ul>
      </section>

      <!-- Active Content Frame -->
      <section class="detail" style="background: radial-gradient(circle at top right, rgba(255,0,127,0.04), transparent 45%);">
        <div v-if="error" class="banner banner-error" style="border-color: rgba(239, 68, 68, 0.4); box-shadow: 0 0 15px rgba(239,68,68,0.1);">{{ error }}</div>
        <div v-if="successMessage" class="banner banner-success" style="border-color: rgba(16, 185, 129, 0.4); box-shadow: 0 0 15px rgba(16,185,129,0.1);">{{ successMessage }}</div>

        <template v-if="info">
          <!-- Header stats -->
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <div style="display: flex; flex-direction: column; gap: 4px;">
              <span style="font-size: 24px; font-weight: 800; color: #fff; letter-spacing: -0.03em;">
                {{ selected?.modelName ?? selected?.product ?? "Tauri-Драйвер" }}
              </span>
              <span style="font-size: 11px; font-family: ui-monospace, monospace; color: var(--muted);">
                Путь порта: {{ selected?.path }}
              </span>
            </div>
            
            <!-- Battery gauge -->
            <div style="display: flex; align-items: center; gap: 8px; background: rgba(0,0,0,0.5); padding: 8px 16px; border-radius: 20px; border: 1px solid rgba(0, 240, 255, 0.2);">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" :style="{ color: info.batteryLevel > 20 ? 'var(--neon-green)' : 'var(--neon-pink)' }">
                <rect x="2" y="7" width="16" height="10" rx="2" ry="2"/>
                <line x1="22" y1="11" x2="22" y2="13"/>
              </svg>
              <span style="font-weight: 800; color: #fff; font-size: 13px;">{{ info.batteryLevel }}%</span>
              <span v-if="info.chargeStatus === 1" class="muted" style="font-size: 10px; font-weight: 800; color: var(--neon-green); text-shadow: 0 0 8px var(--neon-green);">ЗАРЯДКА</span>
            </div>
          </div>

          <!-- Futuristic Tabs Selector -->
          <nav class="tabs" style="border-bottom-color: rgba(255,255,255,0.08);">
            <button class="tab-btn" :class="{ active: activeTab === 'info' }" @click="switchTab('info')">Параметры</button>
            <button class="tab-btn" :class="{ active: activeTab === 'lighting' }" @click="switchTab('lighting')" :disabled="!ledEffect">Эффекты</button>
            
            <!-- Custom Streaming Tabs -->
            <button class="tab-btn" :class="{ active: activeTab === 'ambient' }" @click="switchTab('ambient')" style="color: var(--neon-cyan);" :disabled="!ledEffect">Эмбиент-Экран</button>
            <button class="tab-btn" :class="{ active: activeTab === 'equalizer' }" @click="switchTab('equalizer')" style="color: var(--neon-purple);" :disabled="!ledEffect">Эквалайзер</button>
            <button class="tab-btn" :class="{ active: activeTab === 'pixelart' }" @click="switchTab('pixelart')" style="color: var(--neon-pink);" :disabled="!ledEffect">Pixel-Art GIF</button>
            
            <button class="tab-btn" :class="{ active: activeTab === 'performance' }" @click="switchTab('performance')" :disabled="!gameMode">Rapid Trigger</button>
            <button class="tab-btn" :class="{ active: activeTab === 'maintenance' }" @click="switchTab('maintenance')">Обслуживание</button>
          </nav>

          <!-- Core custom content window -->
          <div class="card" style="flex: 1 1 auto; background: rgba(12, 14, 25, 0.7); border-color: rgba(255,255,255,0.06); padding: 24px; display: flex; flex-direction: column; gap: 16px;">
            
            <!-- 1. Technical specs properties -->
            <div v-if="activeTab === 'info'" class="grid">
              <div class="cell"><label>Интерфейс ядра</label><b>{{ selected?.modelName ?? "Универсальный" }}</b></div>
              <div class="cell"><label>Версия прошивки</label><b>v{{ info.version }}</b></div>
              <div class="cell"><label>Порт адресации</label><b>{{ hex(info.vendorId) }}:{{ hex(info.productId) }}</b></div>
              <div class="cell"><label>Пользовательский профиль</label><b>Профиль {{ info.currentProfile }}</b></div>
              <div class="cell"><label>Свободно в ROM</label><b>{{ info.romSize }} КБ</b></div>
              <div class="cell"><label>Размер макросов</label><b>{{ info.macroSpaceSize }} байт</b></div>
              <div class="cell"><label>Датчик осей</label><b>0x{{ info.sensor.toString(16).toUpperCase() }}</b></div>
              <div class="cell"><label>Проводной режим</label><b>{{ info.workMode === 1 ? "Wired (Провод)" : info.workMode === 2 ? "2.4G Wireless" : "Bluetooth" }}</b></div>
              <div class="cell"><label>Архитектура</label><b>v{{ info.frameVersion }}</b></div>
              <div class="cell"><label>Спектр драйвера</label><b>v{{ info.lightingVersion }}</b></div>
              <div class="cell"><label>Rapid-Trigger задержка</label><b>{{ info.rtPrecision === 0 ? "Стандартная (1/100)" : "Игровая (1/1000)" }}</b></div>
            </div>

            <!-- 2. Standard Lighting Mode -->
            <div v-else-if="activeTab === 'lighting' && ledEffect" style="display: flex; flex-direction: column; gap: 16px;">
              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));">
                <div style="display: flex; flex-direction: column; gap: 14px;">
                  <div class="form-group">
                    <label>Режим встроенной подсветки</label>
                    <select v-model.number="ledEffect.mode" class="select-input" style="background: rgba(0,0,0,0.5); border-color: rgba(255,255,255,0.08);">
                      <option v-for="mode in lightModes" :key="mode.value" :value="mode.value">
                        {{ mode.label }}
                      </option>
                    </select>
                  </div>

                  <div class="form-group">
                    <label>Яркость диодов</label>
                    <div class="slider-container">
                      <input type="range" min="0" max="5" v-model.number="ledEffect.brightness" class="range-slider range-slider-cyan">
                      <span class="slider-value" style="color: var(--neon-cyan);">{{ ledEffect.brightness * 20 }}%</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label>Скорость волны</label>
                    <div class="slider-container">
                      <input type="range" min="1" max="5" v-model.number="ledEffect.speed" class="range-slider range-slider-cyan">
                      <span class="slider-value" style="color: var(--neon-cyan);">{{ ledEffect.speed * 20 }}%</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label>Направление движения эффекта</label>
                    <select v-model.number="ledEffect.direction" class="select-input" style="background: rgba(0,0,0,0.5); border-color: rgba(255,255,255,0.08);">
                      <option :value="0">Вправо (Слева направо)</option>
                      <option :value="1">Влево (Справа налево)</option>
                      <option :value="2">Вниз (Сверху вниз)</option>
                      <option :value="3">Вверх (Снизу вверх)</option>
                    </select>
                  </div>
                </div>

                <div style="display: flex; flex-direction: column; gap: 20px; justify-content: center;">
                  <div class="color-picker-group">
                    <div class="color-picker-card" style="background: rgba(0,0,0,0.4); border-color: rgba(255,255,255,0.04);">
                      <label>Основной диод</label>
                      <div class="color-picker-wrapper" :style="{ backgroundColor: primaryColor, borderColor: 'var(--neon-pink)' }">
                        <input type="color" v-model="primaryColor" class="color-picker-native">
                      </div>
                      <span style="font-family: ui-monospace, monospace; font-size: 11px; font-weight: 700; color: #fff;">{{ primaryColor.toUpperCase() }}</span>
                    </div>

                    <div class="color-picker-card" style="background: rgba(0,0,0,0.4); border-color: rgba(255,255,255,0.04);">
                      <label>Вторичный</label>
                      <div class="color-picker-wrapper" :style="{ backgroundColor: secondaryColor, borderColor: 'var(--neon-cyan)' }">
                        <input type="color" v-model="secondaryColor" class="color-picker-native">
                      </div>
                      <span style="font-family: ui-monospace, monospace; font-size: 11px; font-weight: 700; color: #fff;">{{ secondaryColor.toUpperCase() }}</span>
                    </div>
                  </div>

                  <div style="height: 48px; border-radius: 12px; border: 1px solid rgba(255,0,127,0.25); display: flex; align-items: center; justify-content: center; position: relative; overflow: hidden; background: #000; box-shadow: inset 0 0 10px rgba(255,0,127,0.15);">
                    <div :style="{ position: 'absolute', top: 0, left: 0, bottom: 0, width: '50%', background: `linear-gradient(to right, ${primaryColor}, transparent)`, opacity: 0.3 }"></div>
                    <div :style="{ position: 'absolute', top: 0, right: 0, bottom: 0, width: '50%', background: `linear-gradient(to left, ${secondaryColor}, transparent)`, opacity: 0.3 }"></div>
                    <span style="font-size: 11px; font-weight: 800; color: #fff; z-index: 1; letter-spacing: 0.1em; text-transform: uppercase;">Встроенная волна активна</span>
                  </div>
                </div>
              </div>

              <div style="display: flex; justify-content: flex-end; border-top: 1px solid rgba(255,255,255,0.06); padding-top: 16px;">
                <button class="btn" :disabled="applying" @click="applyLedEffect" style="background: var(--neon-cyan); box-shadow: 0 0 15px rgba(0,240,255,0.35);">
                  <svg v-if="applying" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="animation: spin 1s linear infinite;">
                    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
                  </svg>
                  <span style="font-weight: 800; color: #000;">{{ applying ? "Сохранение…" : "Сохранить профиль подсветки" }}</span>
                </button>
              </div>
            </div>

            <!-- 3. Screen Sync Mirror / Ambient Sync -->
            <div v-else-if="activeTab === 'ambient'" style="display: flex; flex-direction: column; gap: 16px;" class="card-neon-cyan">
              <span style="font-size: 13px; color: var(--text-bright); line-height: 1.5;">
                Проецируйте картинку с вашего экрана или открытого игрового окна в реальном времени напрямую на физические клавиши клавиатуры на высокой скорости!
              </span>

              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); align-items: center;">
                <div style="display: flex; flex-direction: column; gap: 12px;">
                  <div style="display: flex; align-items: center; gap: 10px;">
                    <div class="logo-glow" :style="{ background: isAmbientActive ? 'var(--neon-green)' : 'var(--neon-pink)', boxShadow: isAmbientActive ? '0 0 12px var(--neon-green)' : 'none' }"></div>
                    <span style="font-size: 13px; font-weight: 800;">Статус зеркалирования: <b :style="{ color: isAmbientActive ? 'var(--neon-green)' : 'var(--neon-pink)' }">{{ isAmbientActive ? 'АКТИВНО' : 'ВЫКЛЮЧЕНО' }}</b></span>
                  </div>
                  
                  <div style="font-size: 11px; color: var(--muted);">
                    * Рекомендуется выбрать конкретную вкладку или игровое окно при запросе совместного доступа к экрану для максимальной точности цветовых оттенков.
                  </div>
                  
                  <div style="display: flex; gap: 10px; margin-top: 10px;">
                    <button v-if="!isAmbientActive" class="btn" @click="startAmbientSync" style="background: var(--neon-cyan); box-shadow: 0 0 15px rgba(0,240,255,0.3); color: #000; font-weight: 800; width: 100%; justify-content: center;">
                      Включить зеркало
                    </button>
                    <button v-else class="btn btn-danger" @click="stopAmbientSync" style="width: 100%; justify-content: center;">
                      Остановить трансляцию
                    </button>
                  </div>
                </div>

                <div class="ambient-monitor" style="border-color: rgba(0, 240, 255, 0.2);">
                  <div class="ambient-monitor-glow" :style="{ background: isAmbientActive ? 'radial-gradient(circle at center, rgba(0, 240, 255, 0.25) 0%, transparent 70%)' : 'none' }"></div>
                  <svg v-if="!isAmbientActive" width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="color: rgba(255,255,255,0.15);">
                    <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                    <line x1="8" y1="21" x2="16" y2="21"/>
                    <line x1="12" y1="17" x2="12" y2="21"/>
                  </svg>
                  <div v-else style="display: flex; flex-direction: column; align-items: center; gap: 6px; z-index: 1;">
                    <span style="font-size: 11px; font-weight: 800; color: var(--neon-cyan); text-transform: uppercase; letter-spacing: 0.1em; text-shadow: 0 0 10px rgba(0,240,255,0.4);">Идет стриминг потока</span>
                    <span style="font-size: 10px; color: var(--muted); font-family: ui-monospace, monospace;">~25-30 кадров/сек</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 4. Cyber Equalizer -->
            <div v-else-if="activeTab === 'equalizer'" style="display: flex; flex-direction: column; gap: 16px;" class="card-neon-purple">
              <span style="font-size: 13px; color: var(--text-bright); line-height: 1.5;">
                Сделайте вашу клавиатуру интерактивной с музыкальным эквалайзером! Поддерживаются как встроенные алгоритмы устройства, так и кастомное наложение частотных полос на физический корпус.
              </span>

              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));">
                
                <div style="display: flex; flex-direction: column; gap: 14px;">
                  <div class="form-group">
                    <label>Тип обработки сигнала</label>
                    <div class="mode-grid">
                      <div class="mode-card" :class="{ 'active-pink': eqMode === 'custom' }" @click="eqMode = 'custom'">
                        <span class="mode-card-title">Координатный Градиент</span>
                        <span class="mode-card-desc">Проецирует красивый pink-to-cyan спектр по высоте клавиш на частотах.</span>
                      </div>
                      <div class="mode-card" :class="{ 'active-cyan': eqMode === 'native' }" @click="eqMode = 'native'">
                        <span class="mode-card-title">Hardware Эквалайзер</span>
                        <span class="mode-card-desc">Запускает встроенную DSP-прошивку со звуковым чипом клавиатуры.</span>
                      </div>
                    </div>
                  </div>

                  <div v-if="eqMode === 'native'" style="display: flex; flex-direction: column; gap: 12px; padding: 10px; background: rgba(0,0,0,0.2); border-radius: 8px; border: 1px solid rgba(255,255,255,0.03);">
                    <div class="form-group" style="margin-bottom: 8px;">
                      <label>Встроенный стиль DSP</label>
                      <select v-model.number="eqTheme" class="select-input" style="background: rgba(0,0,0,0.5);">
                        <option :value="0">Водопад (Cascade Bounce)</option>
                        <option :value="1">Энергетический спектр (Linear Spline)</option>
                        <option :value="2">Пульс дыхания (Radial Pulse)</option>
                      </select>
                    </div>
                    <div class="form-group" style="margin-bottom: 8px;">
                      <label>Скорость фильтрации</label>
                      <input type="range" min="1" max="5" v-model.number="eqSpeed" class="range-slider">
                    </div>
                    <div class="form-group" style="margin-bottom: 0;">
                      <label>Сила яркости диодов</label>
                      <input type="range" min="0" max="5" v-model.number="eqBrightness" class="range-slider">
                    </div>
                  </div>

                  <div style="display: flex; gap: 10px; margin-top: 10px;">
                    <button v-if="!isEqActive" class="btn" @click="startEqualizer" style="background: var(--neon-purple); box-shadow: 0 0 15px rgba(157,0,255,0.4); font-weight: 800; width: 100%; justify-content: center;">
                      Запустить визуализатор
                    </button>
                    <button v-else class="btn btn-danger" @click="stopEqualizer" style="width: 100%; justify-content: center;">
                      Выключить эквалайзер
                    </button>
                  </div>
                </div>

                <!-- Animated Equalizer Columns Preview -->
                <div class="eq-bars" style="border-color: rgba(157,0,255,0.2); background: rgba(0,0,0,0.5);">
                  <div
                    v-for="(val, index) in eqBarsData"
                    :key="index"
                    class="eq-bar"
                    :style="{ 
                      height: val + '%', 
                      background: `linear-gradient(to top, var(--neon-purple) 0%, ${eqMode === 'custom' ? 'var(--neon-pink)' : 'var(--neon-cyan)'} 100%)`,
                      boxShadow: val > 50 ? `0 0 12px ${eqMode === 'custom' ? 'var(--neon-pink)' : 'var(--neon-cyan)'}` : 'none'
                    }"
                  ></div>
                </div>

              </div>
            </div>

            <!-- 5. GIF Backlight Pixel Art -->
            <div v-else-if="activeTab === 'pixelart'" style="display: flex; flex-direction: column; gap: 16px;" class="card-neon-pink">
              <span style="font-size: 13px; color: var(--text-bright); line-height: 1.5;">
                Превратите клавиатуру в ретро-экран для пиксель-арта! Перетащите или загрузите анимированный GIF-файл, и драйвер автоматически разложит анимацию по физическому диодному полотну клавиш.
              </span>

              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); align-items: center;">
                
                <div style="display: flex; flex-direction: column; gap: 14px;">
                  <!-- Drag and Drop area -->
                  <div 
                    class="dropzone" 
                    :class="{ active: isGifPlaying }"
                    @dragover.prevent
                    @drop="handleGifDrop"
                    @click="fileInput?.click()"
                    style="border-color: rgba(255,0,127,0.3);"
                  >
                    <input type="file" ref="fileInput" accept="image/gif" @change="handleGifUpload" style="display: none;">
                    <div class="dropzone-icon" style="color: var(--neon-pink);">👾</div>
                    <span v-if="!uploadedGifName" style="font-size: 12px; font-weight: 700; color: #fff;">Перетащите файл .gif или нажмите для выбора</span>
                    <span v-else style="font-size: 12px; font-weight: 700; color: var(--neon-pink); text-shadow: 0 0 10px rgba(255,0,127,0.2);">{{ uploadedGifName }}</span>
                  </div>

                  <div style="display: flex; gap: 10px; margin-top: 10px;" v-if="gifFrames.length > 0">
                    <button v-if="!isGifPlaying" class="btn" @click="startGifPlayback" style="background: var(--neon-pink); box-shadow: 0 0 15px rgba(255,0,127,0.35); font-weight: 800; width: 100%; justify-content: center;">
                      Запустить проигрывание
                    </button>
                    <button v-else class="btn btn-secondary" @click="stopGifPlayback" style="width: 100%; justify-content: center; border-color: var(--neon-pink); color: var(--neon-pink);">
                      Пауза
                    </button>
                  </div>
                </div>

                <div class="ambient-monitor" style="border-color: rgba(255,0,127,0.25);">
                  <div class="ambient-monitor-glow" :style="{ background: isGifPlaying ? 'radial-gradient(circle at center, rgba(255,0,127,0.25) 0%, transparent 70%)' : 'none' }"></div>
                  <div v-if="!isGifPlaying" style="color: rgba(255,255,255,0.15); display: flex; flex-direction: column; align-items: center; gap: 8px;">
                    <span style="font-size: 28px;">📼</span>
                    <span style="font-size: 11px;">Анимация не загружена</span>
                  </div>
                  <div v-else style="display: flex; flex-direction: column; align-items: center; gap: 6px; z-index: 1;">
                    <span style="font-size: 11px; font-weight: 800; color: var(--neon-pink); text-shadow: 0 0 10px var(--neon-pink); letter-spacing: 0.1em; text-transform: uppercase;">ИДЕТ СТРИМИНГ GIF ПИКСЕЛЕЙ</span>
                    <span style="font-size: 10px; color: var(--muted); font-family: ui-monospace, monospace;">Кадр: {{ activeGifFrameIndex + 1 }} / {{ gifFrames.length }}</span>
                  </div>
                </div>

              </div>
            </div>

            <!-- 6. Rapid Trigger / Gaming Mode -->
            <div v-else-if="activeTab === 'performance' && gameMode" style="display: flex; flex-direction: column; gap: 16px;">
              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));">
                <div style="display: flex; flex-direction: column; gap: 14px;">
                  <div class="form-group">
                    <label>Частота шины опроса (Report Rate)</label>
                    <select v-model.number="gameMode.reportRate" class="select-input" style="background: rgba(0,0,0,0.5);">
                      <option :value="1">125 Гц (Минимальный отклик)</option>
                      <option :value="2">250 Гц (Офисный отклик)</option>
                      <option :value="3">500 Гц (Баланс частоты)</option>
                      <option :value="4">1000 Гц (Киберспорт Rapid-Trigger)</option>
                    </select>
                  </div>

                  <div class="form-group">
                    <label>Раскладка ОС</label>
                    <select v-model.number="gameMode.systemMode" class="select-input" style="background: rgba(0,0,0,0.5);">
                      <option :value="0">Windows OS Layout</option>
                      <option :value="1">macOS Apple Layout</option>
                    </select>
                  </div>

                  <div class="form-group">
                    <label>Энергосберегающий режим сна (Таймаут)</label>
                    <div class="slider-container">
                      <input type="range" min="1" max="60" v-model.number="gameMode.sleepTime" class="range-slider">
                      <span class="slider-value">{{ gameMode.sleepTime }} мин</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <label> Debounce задержка клавиш (Анти-дребезг)</label>
                    <div class="slider-container">
                      <input type="range" min="1" max="20" v-model.number="gameMode.keyDelay" class="range-slider">
                      <span class="slider-value">{{ gameMode.keyDelay }} мс</span>
                    </div>
                  </div>
                </div>

                <div style="display: flex; flex-direction: column; gap: 12px;">
                  <div class="switch-container" style="background: rgba(0,0,0,0.2);">
                    <div class="switch-info">
                      <span class="switch-title" style="font-size: 13px;">Пробуждение нажатием</span>
                      <span class="switch-desc">Клавиатура спит полностью до нажатия любой клавиши.</span>
                    </div>
                    <label class="switch">
                      <input type="checkbox" v-model="gameMode.singleKeyWakeup" :true-value="1" :false-value="0">
                      <span class="slider"></span>
                    </label>
                  </div>

                  <div class="switch-container" style="background: rgba(0,0,0,0.2);">
                    <div class="switch-info">
                      <span class="switch-title" style="font-size: 13px;">Калибровка магнитных переключателей</span>
                      <span class="switch-desc">Авто-калибровка датчиков Холла при перепадах температур.</span>
                    </div>
                    <label class="switch">
                      <input type="checkbox" v-model="gameMode.autoCalibration" :true-value="1" :false-value="0">
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

              <div style="display: flex; justify-content: flex-end; border-top: 1px solid rgba(255,255,255,0.06); padding-top: 16px;">
                <button class="btn" :disabled="applying" @click="applyGameMode" style="background: var(--neon-cyan); box-shadow: 0 0 15px rgba(0,240,255,0.35);">
                  <svg v-if="applying" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="animation: spin 1s linear infinite;">
                    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
                  </svg>
                  <span style="font-weight: 800; color: #000;">{{ applying ? "Запись…" : "Сохранить параметры производительности" }}</span>
                </button>
              </div>
            </div>

            <!-- 7. Emergency Maintenance -->
            <div v-else-if="activeTab === 'maintenance'" style="display: flex; flex-direction: column; gap: 16px;">
              <span style="font-size: 13px; color: var(--muted); line-height: 1.5;">
                Панель аварийного восстановления прошивки и сброса сохраненных калибровочных данных. Воспользуйтесь функциями, если клавиши реагируют некорректно.
              </span>

              <div class="grid" style="grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));">
                <div class="cell" style="justify-content: space-between; border-color: rgba(239, 68, 68, 0.2); background: rgba(239,68,68,0.02);">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 800; color: #fff; font-size: 13px;">Заводской сброс всех фреймов</span>
                    <span class="muted" style="font-size: 11px;">Сброс абсолютно всех раскладок биндинга клавиш, эффектов и очистка макро-файлов в ROM.</span>
                  </div>
                  <button class="btn btn-danger" style="width: 100%; justify-content: center;" :disabled="applying" @click="triggerReset(255)">
                    Сбросить всё (Reset EEPROM)
                  </button>
                </div>

                <div class="cell" style="justify-content: space-between; border-color: rgba(245, 158, 11, 0.2); background: rgba(245,158,11,0.02);">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 800; color: #fff; font-size: 13px;">Сбросить калибровку магнитов</span>
                    <span class="muted" style="font-size: 11px;">Полностью удаляет калибровочные таблицы магнитных осей для Rapid Trigger переключателей.</span>
                  </div>
                  <button class="btn btn-secondary" style="width: 100%; justify-content: center; border-color: var(--neon-yellow); color: var(--neon-yellow);" :disabled="applying" @click="triggerReset(5)">
                    Удалить калибровку
                  </button>
                </div>

                <div class="cell" style="justify-content: space-between; background: rgba(255,255,255,0.01);">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 800; color: #fff; font-size: 13px;">Сбросить диодные профили</span>
                    <span class="muted" style="font-size: 11px;">Возврат дефолтных системных анимаций и восстановление заводских спектров.</span>
                  </div>
                  <button class="btn btn-secondary" style="width: 100%; justify-content: center;" :disabled="applying" @click="triggerReset(2)">
                    Сбросить подсветку
                  </button>
                </div>

                <div class="cell" style="justify-content: space-between; background: rgba(255,255,255,0.01);">
                  <div style="display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px;">
                    <span style="font-weight: 800; color: #fff; font-size: 13px;">Очистить биндинги кнопок</span>
                    <span class="muted" style="font-size: 11px;">Стирает кастомную раскладку и переназначает все клавиши на дефолтные ANSI стандарты.</span>
                  </div>
                  <button class="btn btn-secondary" style="width: 100%; justify-content: center;" :disabled="applying" @click="triggerReset(1)">
                    Сбросить бинды клавиш
                  </button>
                </div>
              </div>
            </div>

            <!-- --- EXTREMELY PREMIUM 1:1 INTERACTIVE VISUAL KEYBOARD LAYOUT PREVIEW --- -->
            <div style="margin-top: auto; border-top: 1px solid rgba(255,255,255,0.06); padding-top: 24px; display: flex; flex-direction: column; gap: 12px;">
              <span style="font-size: 11px; text-transform: uppercase; letter-spacing: 0.1em; color: var(--muted); font-weight: 800; text-align: center;">ИНТЕРАКТИВНЫЙ МОНИТОРИНГ ПОДСВЕТКИ (LIVE KEYBOARD PROJECTION)</span>
              
              <div class="virtual-keyboard">
                <div 
                  v-for="(row, rIdx) in keyboardRows" 
                  :key="rIdx" 
                  class="keyboard-row"
                >
                  <div
                    v-for="key in row"
                    :key="key.idx"
                    class="key-cap"
                    :style="{ 
                      width: (key.w * 40) + 'px', 
                      height: (key.h ? (key.h * 40 + (key.h - 1) * 5) : 40) + 'px',
                      textShadow: liveKeyColors[key.idx] ? `0 0 8px ${liveKeyColors[key.idx]}` : 'none',
                      color: liveKeyColors[key.idx] ? '#fff' : '#a9b1d6',
                      borderColor: liveKeyColors[key.idx] ? 'rgba(255,255,255,0.15)' : 'rgba(255,255,255,0.05)'
                    }"
                  >
                    <span style="font-size: 9px; pointer-events: none; margin-top: -4px;">{{ key.label }}</span>
                    <!-- Glowing custom LED projection -->
                    <div 
                      class="key-cap-led" 
                      :style="{ 
                        backgroundColor: liveKeyColors[key.idx] || 'transparent',
                        boxShadow: liveKeyColors[key.idx] ? `0 -1px 12px 2px ${liveKeyColors[key.idx]}` : 'none'
                      }"
                    ></div>
                  </div>
                </div>
              </div>
            </div>

          </div>
        </template>

        <!-- Connected Splash -->
        <div v-else-if="!error" class="muted-hint">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="margin-bottom: 16px; opacity: 0.6; color: var(--neon-pink); filter: drop-shadow(0 0 10px rgba(255,0,127,0.3));">
            <rect x="2" y="4" width="20" height="16" rx="2" ry="2"/>
            <path d="M6 8h.01M10 8h.01M14 8h.01M18 8h.01M6 12h.01M10 12h.01M14 12h.01M18 12h.01M7 16h10"/>
          </svg>
          <p style="font-size: 18px; font-weight: 800; color: var(--text-bright); margin: 0 0 4px; letter-spacing: -0.02em;">ИНТЕРФЕЙС УСТРОЙСТВА НЕ АКТИВЕН</p>
          <p style="font-size: 13px; margin: 0; color: var(--muted);">Выберите клавиатуру в боковом меню слева или выполните быстрое сканирование для калибровки драйвера.</p>
        </div>
      </section>
    </main>
  </div>
</template>

<style>
/* Custom animations */
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
