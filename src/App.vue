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
  startLedStream,
  pushLedFrame,
  stopLedStream,
  sendMusicData,
  getHelperConfig,
  setHelperConfig,
  installHelperAutostart,
  uninstallHelperAutostart,
  getMagneticAxisRt,
  setMagneticAxisRt,
  calibrationStart,
  calibrationFinish,
  pollCalibrationSample,
  isWindowsHost,
  listDisplays,
  pickGifFile,
  type DisplayInfo,
  type DeviceSummary,
  type DeviceInfo,
  type GameMode,
  type LedEffect,
  type LedColor,
  type MagneticAxisRT,
  type CalibrationSample
} from "./lib/api";

// --- Hardware Key Backlight Layout Matrix ---
// --- Hardware Key Backlight Layout Matrix ---
interface KeyboardKey {
  className: string;
  value: number;
  name: string;
  keyCode: number;
  key: string;
}

const rawKeyboardList: KeyboardKey[][] = [[{className:"",value:0,name:"Esc",keyCode:41,key:"Escape"},{className:"ml-15",value:1,name:"F1",keyCode:58,key:"F1"},{className:"",value:2,name:"F2",keyCode:59,key:"F2"},{className:"",value:3,name:"F3",keyCode:60,key:"F3"},{className:"",value:4,name:"F4",keyCode:61,key:"F4"},{className:"ml-auto",value:5,name:"F5",keyCode:62,key:"F5"},{className:"",value:6,name:"F6",keyCode:63,key:"F6"},{className:"",value:7,name:"F7",keyCode:64,key:"F7"},{className:"",value:8,name:"F8",keyCode:65,key:"F8"},{className:"ml-auto",value:9,name:"F9",keyCode:66,key:"F9"},{className:"",value:10,name:"F10",keyCode:67,key:"F10"},{className:"",value:11,name:"F11",keyCode:68,key:"F11"},{className:"mr-5",value:12,name:"F12",keyCode:69,key:"F12"},{className:"mr-5",value:106,name:"Del",keyCode:76,key:"Delete"},{className:"",value:103,name:"Insert",keyCode:73,key:"Insert"},{className:"",value:99,name:"Print",keyCode:70,key:"PrintScreen"},{className:"",value:100,name:"Scroll",keyCode:71,key:"ScrollLock"},{className:"",value:102,name:"Pause",keyCode:72,key:"Pause"}],[{className:"",value:16,name:"` ~",keyCode:53,key:"Backquote"},{className:"",value:17,name:"1 !",keyCode:30,key:"Digit1"},{className:"",value:18,name:"2 @",keyCode:31,key:"Digit2"},{className:"",value:19,name:"3 #",keyCode:32,key:"Digit3"},{className:"",value:20,name:"4 $",keyCode:33,key:"Digit4"},{className:"",value:21,name:"5 %",keyCode:34,key:"Digit5"},{className:"",value:22,name:"6 ^",keyCode:35,key:"Digit6"},{className:"",value:23,name:"7 &",keyCode:36,key:"Digit7"},{className:"",value:24,name:"8 *",keyCode:37,key:"Digit8"},{className:"",value:25,name:"9 (",keyCode:38,key:"Digit9"},{className:"",value:26,name:"0 )",keyCode:39,key:"Digit0"},{className:"",value:27,name:"ˉ -",keyCode:45,key:"Minus"},{className:"",value:28,name:"= +",keyCode:46,key:"Equal"},{className:"w-30",value:92,name:"Back",keyCode:42,key:"Backspace"},{className:"ml-5",value:104,name:"Home",keyCode:74,key:"Home"},{className:"ml-5",value:29,name:"Num Lock",keyCode:83,key:"NumLock"},{className:"",value:30,name:"Num /",keyCode:84,key:"NumpadDivide"},{className:"",value:31,name:"Num *",keyCode:85,key:"NumpadMultiply"},{className:"",value:109,name:"Num -",keyCode:86,key:"Numpadsubtract"}],[{className:"w-22",value:32,name:"Tab",keyCode:43,key:"Tab"},{className:"",value:33,name:"Q",keyCode:20,key:"KeyQ"},{className:"",value:34,name:"W",keyCode:26,key:"KeyW"},{className:"",value:35,name:"E",keyCode:8,key:"KeyE"},{className:"",value:36,name:"R",keyCode:21,key:"KeyR"},{className:"",value:37,name:"T",keyCode:23,key:"KeyT"},{className:"",value:38,name:"Y",keyCode:28,key:"KeyY"},{className:"",value:39,name:"U",keyCode:24,key:"KeyU"},{className:"",value:40,name:"I",keyCode:12,key:"KeyI"},{className:"",value:41,name:"O",keyCode:18,key:"KeyO"},{className:"",value:42,name:"P",keyCode:19,key:"KeyP"},{className:"",value:43,name:"[ {",keyCode:47,key:"BracketLeft"},{className:"",value:44,name:"] }",keyCode:48,key:"BracketRight"},{className:"w-23",value:60,name:"\\ | ",keyCode:49,key:"Backslash"},{className:"ml-auto mr-5",value:105,name:"PgUp",keyCode:75,key:"PageUp"},{className:"",value:45,name:"Num 7",keyCode:95,key:"Numpad7"},{className:"",value:46,name:"Num 8",keyCode:96,key:"Numpad8"},{className:"",value:47,name:"Num 9",keyCode:97,key:"Numpad9"},{className:"h-30",value:110,name:"Num +",keyCode:87,key:"NumpadAdd"}],[{className:"w-27 mt--15",value:48,name:"Caps",keyCode:57,key:"CapsLock"},{className:"mt--15",value:49,name:"A",keyCode:4,key:"KeyA"},{className:"mt--15",value:50,name:"S",keyCode:22,key:"KeyS"},{className:"mt--15",value:51,name:"D",keyCode:7,key:"KeyD"},{className:"mt--15",value:52,name:"F",keyCode:9,key:"KeyF"},{className:"mt--15",value:53,name:"G",keyCode:10,key:"KeyG"},{className:"mt--15",value:54,name:"H",keyCode:11,key:"KeyH"},{className:"mt--15",value:55,name:"J",keyCode:13,key:"KeyJ"},{className:"mt--15",value:56,name:"K",keyCode:14,key:"KeyK"},{className:"mt--15",value:57,name:"L",keyCode:15,key:"KeyL"},{className:"mt--15",value:58,name:"; :",keyCode:51,key:"Semicolon"},{className:"mt--15",value:59,name:`' "`,keyCode:52,key:"Quote"},{className:"mt--15 mr-5 w-33",value:76,name:"Enter",keyCode:40,key:"Enter"},{className:"mt--15",value:108,name:"PgDn",keyCode:78,key:"PageDown"},{className:"ml-5 mt--15",value:61,name:"Num 4",keyCode:92,key:"Numpad4"},{className:"mt--15",value:62,name:"Num 5",keyCode:93,key:"Numpad5"},{className:"mr-15 mt--15",value:63,name:"Num 6",keyCode:94,key:"Numpad6"}],[{className:"w-34",value:64,name:"L-Shift",keyCode:225,key:"ShiftLeft"},{className:"",value:65,name:"Z",keyCode:29,key:"KeyZ"},{className:"",value:66,name:"X",keyCode:27,key:"KeyX"},{className:"",value:67,name:"C",keyCode:6,key:"KeyC"},{className:"",value:68,name:"V",keyCode:25,key:"KeyV"},{className:"",value:69,name:"B",keyCode:5,key:"KeyB"},{className:"",value:70,name:"N",keyCode:17,key:"KeyN"},{className:"",value:71,name:"M",keyCode:16,key:"KeyM"},{className:"",value:72,name:", <",keyCode:54,key:"Comma"},{className:"",value:73,name:". >",keyCode:55,key:"Period"},{className:"",value:74,name:"/ ?",keyCode:56,key:"Slash"},{className:"w-25",value:75,name:"R-Shift",keyCode:229,key:"ShiftRight"},{className:"mr-20",value:90,name:"↑",keyCode:82,key:"ArrowUp"},{className:"ml-5",value:77,name:"Num 1",keyCode:89,key:"Numpad1"},{className:"",value:78,name:"Num 2",keyCode:90,key:"Numpad2"},{className:"",value:79,name:"Num 3",keyCode:91,key:"Numpad3"},{className:"h-30",value:95,name:"Num Enter",keyCode:88,key:"NumpadEnter"}],[{className:"w-18 mt--15",value:80,name:"L-Ctrl",keyCode:224,key:"ControlLeft"},{className:"w-18 mt--15",value:81,name:"L-Win",keyCode:227,key:"MetaLeft"},{className:"w-18 mt--15",value:82,name:"L-Alt",keyCode:226,key:"AltLeft"},{className:"flex-1 mt--15",value:83,name:"Spacebar",keyCode:44,key:"Space"},{className:"w-18 mt--15",value:85,name:"Fn",keyCode:175,key:"-1"},{className:"w-18 mt--15",value:87,name:"R-Ctrl",keyCode:228,key:"ControlRight"},{className:"ml-5 mt--15",value:88,name:"←",keyCode:80,key:"ArrowLeft"},{className:"mt--15",value:89,name:"↓",keyCode:81,key:"ArrowDown"},{className:"mt--15 mr-5",value:91,name:"→",keyCode:79,key:"ArrowRight"},{className:"ml-5 w-29 mt--15",value:93,name:"Num 0",keyCode:98,key:"Numpad0"},{className:"mr-15 mt--15",value:94,name:"Num .",keyCode:99,key:"NumpadDecimal"}]];

// Flat list of keys with physical (x, y) coordinates for spatial visualizer algorithms
const keysList: { label: string; idx: number; x: number; y: number; w: number; h?: number }[] = [];

rawKeyboardList.forEach((row, y) => {
  let currentX = 0;
  row.forEach((key) => {
    let w = 1.0;
    if (key.className.includes("w-18")) w = 1.25;
    else if (key.className.includes("w-22")) w = 1.5;
    else if (key.className.includes("w-23")) w = 1.6;
    else if (key.className.includes("w-25")) w = 1.75;
    else if (key.className.includes("w-27")) w = 1.9;
    else if (key.className.includes("w-29")) w = 2.0;
    else if (key.className.includes("w-30")) w = 2.15;
    else if (key.className.includes("w-33")) w = 2.35;
    else if (key.className.includes("w-34")) w = 2.4;
    else if (key.className.includes("flex-1")) w = 6.25;

    let ml = 0.0;
    if (key.className.includes("ml-15")) ml = 1.07;
    else if (key.className.includes("ml-5")) ml = 0.35;
    else if (key.className.includes("ml-auto")) ml = 0.5;

    let mr = 0.0;
    if (key.className.includes("mr-15")) mr = 1.07;
    else if (key.className.includes("mr-5")) mr = 0.35;
    else if (key.className.includes("mr-20")) mr = 1.42;

    let h = 1.0;
    if (key.className.includes("h-30")) h = 2.0;

    currentX += ml;
    const x = currentX;
    currentX += w;
    currentX += mr;

    keysList.push({
      label: key.name,
      idx: key.value,
      x: x,
      y: y,
      w: w,
      h: h
    });
  });
});

const layoutWidth = Math.max(...keysList.map(k => k.x + k.w), 19.5);
const layoutHeight = Math.max(...keysList.map(k => k.y), 5);

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
const gifScaleMode = ref("cover"); // "cover" or "contain"
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
  // Calibration session must end before leaving the tab — otherwise the firmware stays in
  // calibration mode and ignores normal input.
  if (calibrationActive.value && tab !== 'magnetic') cancelCalibration();
  activeTab.value = tab;

  if (tab === 'lighting') {
    updateStaticBacklight();
  } else {
    // Reset preview to default dark purple state for custom streaming visuals
    for (const k of keysList) {
      liveKeyColors.value[k.idx] = "rgba(255,255,255,0.02)";
    }
  }

  if (tab === 'magnetic' && !magneticKeys.value) {
    loadMagneticAxis();
  }
}

// --- Active Backlight Streaming Bridge with On-Screen Preview Mirror ---
// Hands the frame to the Rust daemon (O(1) store, no device I/O here). The daemon owns the
// device and writes at a steady cadence independent of this webview's timers, so frames never
// pile up behind a blocking invoke. The daemon must be running (startLedStream) first.
function streamBacklight(frame: LedColor[]) {
  if (!selected.value) return;
  // 1. Instantly update the on-screen keyboard preview.
  for (const k of frame) {
    liveKeyColors.value[k.idx] = `rgb(${k.r}, ${k.g}, ${k.b})`;
  }
  // 2. Push the latest frame to the daemon (fire-and-forget).
  pushLedFrame(frame).catch(e => console.error("pushLedFrame failed:", e));
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
    // Start the Rust streaming daemon (owns the device, enters custom mode, paces writes).
    await startLedStream(selected.value.path);

    ambientInterval = setInterval(() => {
      if (!ambientVideo || ambientVideo.readyState < ambientVideo.HAVE_CURRENT_DATA) return;
      ctx.drawImage(ambientVideo, 0, 0, ambientCanvas!.width, ambientCanvas!.height);
      const imgData = ctx.getImageData(0, 0, ambientCanvas!.width, ambientCanvas!.height);
      const data = imgData.data;

      const frame: LedColor[] = [];
      for (const key of keysList) {
        // Map 2D physical key matrix space to downsampled coordinates
        const px = Math.min(63, Math.round((key.x / layoutWidth) * 63));
        const py = Math.min(35, Math.round((key.y / layoutHeight) * 35));

        const offset = (py * 64 + px) * 4;
        const r = data[offset];
        const g = data[offset + 1];
        const b = data[offset + 2];

        frame.push({ idx: key.idx, r, g, b });
      }
      // O(1) hand-off to the daemon; the daemon does the actual paced device writes.
      streamBacklight(frame);
    }, 33); // ~30 FPS capture; the daemon de-dupes and paces actual writes
    
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
  stopLedStream().catch(() => {});
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
          const colIndex = Math.min(20, Math.floor((key.x / layoutWidth) * 20));
          const amp = amplitudes[colIndex] || 0;
          
          const heightRatio = amp / 255;
          const keyHeightRatio = (layoutHeight - key.y) / layoutHeight;
          
          if (keyHeightRatio <= heightRatio) {
            // Neon pink-to-cyan audio column gradient
            const ratio = (layoutHeight - key.y) / layoutHeight;
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

async function startGifPlayback() {
  if (gifFrames.value.length === 0 || !selected.value) return;
  isGifPlaying.value = true;
  activeGifFrameIndex.value = 0;
  // Drive the GIF through the same Rust daemon as screen-sync (owns the device, paces writes).
  await startLedStream(selected.value.path);
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
    const u = key.x / layoutWidth;
    const v = key.y / layoutHeight;
    
    let u_mapped, v_mapped, in_bounds = true;
    
    if (gifScaleMode.value === "contain") {
      // Contain: Fit entire GIF inside keyboard grid, pad out-of-bounds keys
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
    } else {
      // Cover: Zoom and crop to fill 100% of keys
      if (AR_gif > AR_kb) {
        const scale = AR_kb / AR_gif;
        u_mapped = (u - 0.5) * scale + 0.5;
        v_mapped = v;
      } else {
        const scale = AR_gif / AR_kb;
        u_mapped = u;
        v_mapped = (v - 0.5) * scale + 0.5;
      }
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
      // Ambient dark background padding for Contain mode
      frame.push({ idx: key.idx, r: 12, g: 8, b: 24 });
    }
  }
  
  streamBacklight(frame);

  activeGifFrameIndex.value = (activeGifFrameIndex.value + 1) % gifFrames.value.length;
  gifTimeout = setTimeout(playNextGifFrame, frameData.delay);
}

function stopGifPlayback() {
  isGifPlaying.value = false;
  stopLedStream().catch(() => {});
  if (gifTimeout) {
    clearTimeout(gifTimeout);
    gifTimeout = null;
  }
}

// Effect catalog matched against the upstream bundle's `defalutLightingModeList`
// (lighting-*.js). Previous version used `lightBoxModeList` which is the *side strip*
// catalog, not the main matrix — that mismatch is why mode 2 actually behaved as press-
// reactive (Single Point On) instead of a static fill on AK980 MAX.
//
// `showSpeed` / `showDirection` / `showColor` mirror the upstream UI gating so a future
// pass can grey out sliders the firmware ignores for that mode.
const lightModes = [
  { value: 1,   label: "Статичная заливка (Static Bright)",   showSpeed: false, showDirection: false, showColor: true  },
  { value: 2,   label: "Реактивная — зажигать (Press On)",    showSpeed: true,  showDirection: false, showColor: true  },
  { value: 3,   label: "Реактивная — гасить (Press Off)",     showSpeed: true,  showDirection: false, showColor: true  },
  { value: 4,   label: "Звёздное небо (Starry Sky)",          showSpeed: true,  showDirection: false, showColor: true  },
  { value: 5,   label: "Снегопад (Snowfall)",                 showSpeed: true,  showDirection: false, showColor: true  },
  { value: 6,   label: "Цветение (Floral)",                   showSpeed: true,  showDirection: false, showColor: false },
  { value: 7,   label: "Дыхание (Dynamic Breathing)",         showSpeed: true,  showDirection: false, showColor: true  },
  { value: 8,   label: "Спектральный цикл (Spectrum Cycle)",  showSpeed: true,  showDirection: false, showColor: false },
  { value: 9,   label: "Цветной фонтан (Color Fountain)",     showSpeed: true,  showDirection: false, showColor: true  },
  { value: 10,  label: "Цветной поток (Colorful Interchange)",showSpeed: true,  showDirection: true,  showColor: true  },
  { value: 11,  label: "По волнам (Flowing with Waves)",      showSpeed: true,  showDirection: true,  showColor: true  },
  { value: 12,  label: "Перевал (Turning Peaks)",             showSpeed: true,  showDirection: true,  showColor: true  },
  { value: 13,  label: "Касание-вспышка (One Touch to Fire)", showSpeed: true,  showDirection: false, showColor: true  },
  { value: 14,  label: "Один в двух (Two Birds)",             showSpeed: true,  showDirection: false, showColor: true  },
  { value: 15,  label: "Расходящаяся рябь (Ripples)",         showSpeed: true,  showDirection: false, showColor: true  },
  { value: 16,  label: "Бесконечный поток (Endless Flow)",    showSpeed: true,  showDirection: true,  showColor: true  },
  { value: 17,  label: "Слоистые горы (Layered Mountains)",   showSpeed: true,  showDirection: false, showColor: true  },
  { value: 18,  label: "Дождь и ветер (Gentle Rain)",         showSpeed: true,  showDirection: true,  showColor: true  },
  { value: 19,  label: "Туда-сюда (Back and Forth)",          showSpeed: true,  showDirection: false, showColor: true  },
  { value: 128, label: "Своя подсветка (Custom)",             showSpeed: false, showDirection: false, showColor: true  },
];

// --- Background helper (ajazz-helperd) control ---
const helperMode = ref<string>("off");
const helperGifPath = ref<string>("");
const helperFps = ref<number>(30);
const helperScreenIndex = ref<number | null>(null);
const helperDisplays = ref<DisplayInfo[]>([]);

async function refreshDisplays() {
  try {
    helperDisplays.value = await listDisplays();
    // First time we see the list, default-select the primary monitor so the dropdown isn't
    // blank. The user can change it, and the choice is persisted via helper.json.
    if (helperScreenIndex.value == null && helperDisplays.value.length) {
      const primary = helperDisplays.value.find((d) => d.isPrimary) ?? helperDisplays.value[0];
      helperScreenIndex.value = primary.index;
    }
  } catch { /* non-Windows hosts return [] silently */ }
}

async function pickHelperGif() {
  try {
    const picked = await pickGifFile();
    if (picked) {
      helperGifPath.value = picked;
      await pushHelperConfig();
    }
  } catch (e) {
    error.value = "Не удалось открыть диалог выбора файла: " + String(e);
  }
}

async function pushHelperConfig() {
  try {
    await setHelperConfig({
      mode: helperMode.value,
      devicePath: selected.value?.path ?? null,
      gifPath: helperGifPath.value || null,
      fps: helperFps.value || 30,
      screenIndex: helperScreenIndex.value,
    });
  } catch (e) {
    error.value = "Не удалось записать конфиг фонового сервиса: " + String(e);
  }
}

async function setHelperMode(mode: string) {
  // The background helper drives the device; stop any in-app streaming first to avoid contention.
  if (mode !== "off") {
    stopAmbientSync();
    stopGifPlayback();
  }
  helperMode.value = mode;
  await pushHelperConfig();
  if (mode !== "off") {
    triggerSuccess(mode === "screen" ? "Фоновый стрим экрана включён!" : "Фоновое воспроизведение GIF включено!");
  }
}

async function enableAutostart() {
  try {
    await pushHelperConfig();
    await installHelperAutostart();
    triggerSuccess("Автозапуск фонового сервиса включён.");
  } catch (e) {
    error.value = "Не удалось включить автозапуск: " + String(e);
  }
}

async function disableAutostart() {
  try {
    await uninstallHelperAutostart();
    triggerSuccess("Автозапуск отключён.");
  } catch (e) {
    error.value = "Не удалось отключить автозапуск: " + String(e);
  }
}

// ─── Magnetic-axis Rapid Trigger & calibration ────────────────────────────────────────────────

const magneticKeys = ref<MagneticAxisRT[] | null>(null);
const magneticLoading = ref(false);
// Global ("apply to all") setting controls — applied to every key on save.
const rtGlobal = ref({ triggerKeyStroke: 1.5, pressRT: 0.5, releaseRT: 0.5, isWholeFast: true, isRampageMode: false });

// Hand-picked RT response profiles. Each one shapes how aggressive the key feels: lower stroke /
// RT = earlier activation = faster repeated taps. Curves are 4-point sample arrays used by the
// preset chip sparkline visual.
const rtPresets = [
  {
    id: "gaming",
    name: "Gaming",
    label: "RAPID · 0.3mm",
    desc: "Триггер на полупогружении, минимальный сброс. Для шутеров и платформеров.",
    accent: "var(--neon-pink)",
    values: { triggerKeyStroke: 0.3, pressRT: 0.05, releaseRT: 0.05, isWholeFast: true, isRampageMode: false },
    curve: [0.0, 0.1, 0.85, 1.0],
  },
  {
    id: "balanced",
    name: "Balanced",
    label: "MIDPOINT · 1.0mm",
    desc: "Универсальный профиль: быстрый отклик без ложных нажатий.",
    accent: "var(--neon-cyan)",
    values: { triggerKeyStroke: 1.0, pressRT: 0.2, releaseRT: 0.2, isWholeFast: true, isRampageMode: false },
    curve: [0.0, 0.25, 0.7, 1.0],
  },
  {
    id: "typing",
    name: "Typing",
    label: "FULL · 2.0mm",
    desc: "Глубокий ход, статичный сброс. Привычная «механика» для текста.",
    accent: "var(--neon-purple)",
    values: { triggerKeyStroke: 2.0, pressRT: 0.5, releaseRT: 0.5, isWholeFast: false, isRampageMode: false },
    curve: [0.0, 0.35, 0.55, 1.0],
  },
];
const activePresetId = ref<string | null>(null);

function presetCurvePath(curve: number[]): string {
  // Smooth-ish sparkline through `curve.length` evenly-spaced y values in [0,1] (top-down SVG).
  const w = 100;
  const h = 40;
  const stepX = w / (curve.length - 1);
  const pts = curve.map((y, i) => [i * stepX, h - y * (h - 4) - 2] as const);
  let d = `M ${pts[0][0].toFixed(2)},${pts[0][1].toFixed(2)}`;
  for (let i = 1; i < pts.length; i++) {
    const [px, py] = pts[i - 1];
    const [x, y] = pts[i];
    const cx = (px + x) / 2;
    d += ` Q ${cx.toFixed(2)},${py.toFixed(2)} ${x.toFixed(2)},${y.toFixed(2)}`;
  }
  return d;
}

async function applyPreset(presetId: string) {
  const preset = rtPresets.find((p) => p.id === presetId);
  if (!preset || !selected.value || !info.value) return;
  rtGlobal.value = { ...preset.values };
  activePresetId.value = presetId;
  // If we haven't read the per-key data yet, fetch first — set requires the full 128-key payload.
  if (!magneticKeys.value) {
    await loadMagneticAxis();
    if (!magneticKeys.value) return; // still nothing → user is offline
  }
  await applyRapidTriggerToAll();
}

// ─── SOCD (Simultaneous Opposite Cardinal Directions) ─────────────────────────────────────────
// Stored locally for now: the upstream SET_KEY (cmd 34) byte layout isn't reverse-engineered yet,
// so we keep the user's pairs in localStorage and surface a clear "client-side only" banner. This
// keeps the UX shippable today without writing garbage to the device's binding flash.

interface SocdPair { id: string; k1: number; k2: number; mode: "last" | "k1" | "k2" | "neutral"; }
const SOCD_STORAGE_KEY = "ajazz.socd.pairs";
const socdPairs = ref<SocdPair[]>([]);
const socdK1 = ref<number | null>(null);
const socdK2 = ref<number | null>(null);
const socdMode = ref<SocdPair["mode"]>("last");

const SOCD_MODES: Array<{ id: SocdPair["mode"]; name: string; desc: string }> = [
  { id: "last", name: "Last input", desc: "Срабатывает последняя нажатая клавиша." },
  { id: "k1", name: "Key 1 wins", desc: "K1 всегда подавляет K2." },
  { id: "k2", name: "Key 2 wins", desc: "K2 всегда подавляет K1." },
  { id: "neutral", name: "Anti-Ghost", desc: "Обе клавиши гасятся, пока зажаты вместе." },
];

const keyLabelByValue = computed(() => {
  const m: Record<number, string> = {};
  for (const k of keysList) m[k.idx] = k.label;
  return m;
});

function loadSocdFromStorage() {
  try {
    const raw = window.localStorage.getItem(SOCD_STORAGE_KEY);
    if (raw) socdPairs.value = JSON.parse(raw);
  } catch { /* ignore corrupt blob */ }
}
function persistSocd() {
  try { window.localStorage.setItem(SOCD_STORAGE_KEY, JSON.stringify(socdPairs.value)); } catch {}
}

function addSocdPair() {
  if (socdK1.value == null || socdK2.value == null) return;
  if (socdK1.value === socdK2.value) {
    error.value = "Клавиши SOCD-пары должны быть разными.";
    return;
  }
  socdPairs.value.push({
    id: `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 6)}`,
    k1: socdK1.value, k2: socdK2.value, mode: socdMode.value,
  });
  persistSocd();
  socdK1.value = null; socdK2.value = null;
}
function removeSocdPair(id: string) {
  socdPairs.value = socdPairs.value.filter((p) => p.id !== id);
  persistSocd();
}
function applySocdTemplate(name: "wasd" | "arrows") {
  const tpl = name === "wasd"
    ? [{ k1: 49, k2: 51 }, { k1: 34, k2: 50 }]      // A/D, W/S (key values from rawKeyboardList)
    : [{ k1: 88, k2: 91 }, { k1: 90, k2: 89 }];     // ←/→, ↑/↓
  for (const t of tpl) {
    if (socdPairs.value.some((p) => (p.k1 === t.k1 && p.k2 === t.k2) || (p.k1 === t.k2 && p.k2 === t.k1))) continue;
    socdPairs.value.push({ id: `tpl-${name}-${t.k1}-${t.k2}`, ...t, mode: "last" });
  }
  persistSocd();
}

// ─── Live ADC oscilloscope ─────────────────────────────────────────────────────────────────
// The firmware streams per-key analog samples (cmd 251) the moment calibration mode is active.
// We piggyback on that same stream and render the `currentValue` (raw Hall sample) as a
// rolling trace per touched key. No new protocol surface required.

const SCOPE_WIDTH = 100; // samples per channel kept in the rolling buffer
const adcMonitorActive = ref(false);
const adcTraces = ref<Record<number, number[]>>({});
const adcMaxStroke = ref<Record<number, number>>({});
let adcPollHandle: any = null;
let adcWasInCalibration = false;

async function toggleAdcMonitor() {
  if (adcMonitorActive.value) { stopAdcMonitor(); return; }
  if (!selected.value) return;
  // We hijack the calibration channel: if a real calibration session is already running we just
  // attach to it; otherwise we start one quietly. The scope view doesn't write anything to flash.
  adcWasInCalibration = calibrationActive.value;
  if (!calibrationActive.value) {
    try { await calibrationStart(selected.value.path); } catch (e) { error.value = String(e); return; }
  }
  adcMonitorActive.value = true;
  adcTraces.value = {};
  adcMaxStroke.value = {};
  adcPollHandle = setInterval(async () => {
    if (!selected.value || !adcMonitorActive.value) return;
    try {
      const s = await pollCalibrationSample(selected.value.path);
      if (!s) return;
      const trace = adcTraces.value[s.keyValue] ? [...adcTraces.value[s.keyValue]] : [];
      const normalised = s.maxStroke > 0 ? Math.min(1, s.keyStroke / s.maxStroke) : 0;
      trace.push(normalised);
      if (trace.length > SCOPE_WIDTH) trace.splice(0, trace.length - SCOPE_WIDTH);
      adcTraces.value = { ...adcTraces.value, [s.keyValue]: trace };
      adcMaxStroke.value = { ...adcMaxStroke.value, [s.keyValue]: s.maxStroke };
    } catch { /* transient — skip this tick */ }
  }, 40);
}

function stopAdcMonitor() {
  adcMonitorActive.value = false;
  if (adcPollHandle) { clearInterval(adcPollHandle); adcPollHandle = null; }
  // If we opened the calibration session ourselves, close it cleanly so the firmware exits the
  // ADC streaming flag (`*0x2000047d`). Don't touch it if a wizard owns the session.
  if (!adcWasInCalibration && selected.value) {
    calibrationFinish(selected.value.path).catch(() => {});
  }
}

const SCOPE_COLORS = ["#39ff14", "#00f0ff", "#ff007f", "#dfff00", "#9d00ff", "#ff8800", "#39ffaa", "#ff5577"];
function scopeColor(idx: number, position: number): string {
  return SCOPE_COLORS[position % SCOPE_COLORS.length] ?? "#39ff14";
}
function scopeTracePath(trace: number[]): string {
  if (!trace.length) return "";
  const w = 100, h = 100;
  const stepX = trace.length > 1 ? w / (trace.length - 1) : w;
  return trace.map((y, i) => `${i === 0 ? "M" : "L"} ${(i * stepX).toFixed(2)},${(h - y * h).toFixed(2)}`).join(" ");
}

const activeScopeChannels = computed(() => Object.keys(adcTraces.value).map(Number).sort((a, b) => a - b));

// Calibration session state. `samples` is keyed by physical LED index so we can colour the keys
// directly on the virtual keyboard during a session.
const calibrationActive = ref(false);
const calibrationSamples = ref<Record<number, CalibrationSample>>({});
let calibrationPollHandle: any = null;

// Resolved on mount from the Rust side — navigator.platform returns "Win32" (not "Windows"),
// which breaks naive UA-based detection inside WebView2.
const isHostWindows = ref(false);

const calibratedCount = computed(() =>
  Object.values(calibrationSamples.value).filter((s) => s.calibrationStatus >= 1).length
);

async function loadMagneticAxis() {
  if (!selected.value || !info.value) return;
  magneticLoading.value = true;
  try {
    const keys = await getMagneticAxisRt(selected.value.path, info.value.rtPrecision, info.value.frameVersion);
    magneticKeys.value = keys;
    // Seed the global controls from the first key so the sliders mean something the moment we open.
    if (keys[0]) {
      rtGlobal.value.triggerKeyStroke = keys[0].triggerKeyStroke;
      rtGlobal.value.pressRT = keys[0].pressRT;
      rtGlobal.value.releaseRT = keys[0].releaseRT;
      rtGlobal.value.isWholeFast = keys[0].isWholeFast;
      rtGlobal.value.isRampageMode = keys[0].isRampageMode;
    }
  } catch (e) {
    error.value = "Не удалось прочитать настройки магнитных свичей: " + String(e);
  } finally {
    magneticLoading.value = false;
  }
}

async function applyRapidTriggerToAll() {
  if (!selected.value || !info.value || !magneticKeys.value) return;
  applying.value = true;
  error.value = null;
  try {
    const updated = magneticKeys.value.map((k) => ({
      ...k,
      triggerKeyStroke: rtGlobal.value.triggerKeyStroke,
      pressRT: rtGlobal.value.pressRT,
      releaseRT: rtGlobal.value.releaseRT,
      isWholeFast: rtGlobal.value.isWholeFast,
      isRampageMode: rtGlobal.value.isRampageMode,
    }));
    await setMagneticAxisRt(selected.value.path, updated, info.value.rtPrecision, info.value.frameVersion);
    magneticKeys.value = updated;
    triggerSuccess("Rapid Trigger применён ко всем клавишам.");
  } catch (e) {
    error.value = "Не удалось записать Rapid Trigger: " + String(e);
  } finally {
    applying.value = false;
  }
}

async function startCalibration() {
  if (!selected.value) return;
  // Streaming + calibration would compete for the device; stop any live effects first.
  stopAmbientSync();
  stopEqualizer();
  stopGifPlayback();
  error.value = null;
  calibrationSamples.value = {};
  try {
    await calibrationStart(selected.value.path);
    calibrationActive.value = true;
    triggerSuccess("Калибровка началась — нажмите каждую клавишу до упора.");
    // Poll for samples; the firmware streams them as the user works through the keyboard.
    calibrationPollHandle = setInterval(async () => {
      if (!selected.value || !calibrationActive.value) return;
      try {
        const s = await pollCalibrationSample(selected.value.path);
        if (s) calibrationSamples.value = { ...calibrationSamples.value, [s.keyValue]: s };
      } catch { /* poll failures are transient; the next tick retries */ }
    }, 60);
  } catch (e) {
    calibrationActive.value = false;
    error.value = "Не удалось начать калибровку: " + String(e);
  }
}

async function finishCalibration() {
  if (!selected.value) return;
  calibrationActive.value = false;
  if (calibrationPollHandle) { clearInterval(calibrationPollHandle); calibrationPollHandle = null; }
  try {
    await calibrationFinish(selected.value.path);
    triggerSuccess(`Калибровка сохранена. Откалибровано клавиш: ${calibratedCount.value}.`);
  } catch (e) {
    error.value = "Не удалось завершить калибровку: " + String(e);
  }
}

function cancelCalibration() {
  calibrationActive.value = false;
  if (calibrationPollHandle) { clearInterval(calibrationPollHandle); calibrationPollHandle = null; }
  calibrationSamples.value = {};
  // Still send the OFF packet so the firmware exits calibration mode cleanly.
  if (selected.value) calibrationFinish(selected.value.path).catch(() => {});
}

function calibrationColor(ledIdx: number): string {
  const s = calibrationSamples.value[ledIdx];
  if (!s) return "rgba(255,255,255,0.04)";
  if (s.calibrationStatus >= 1) return "rgba(57,255,20,0.45)"; // green
  if (s.currentValue > 0) return "rgba(223,255,0,0.35)";       // yellow — touched, not done
  return "rgba(255,0,127,0.18)";                                 // pink — waiting
}

onMounted(async () => {
  await refresh();
  loadSocdFromStorage();
  try { isHostWindows.value = await isWindowsHost(); } catch { /* fall back to disabled UI */ }
  try {
    const cfg = await getHelperConfig();
    helperMode.value = cfg.mode;
    helperGifPath.value = cfg.gifPath ?? "";
    helperFps.value = cfg.fps || 30;
    helperScreenIndex.value = cfg.screenIndex ?? null;
  } catch { /* helper config optional */ }
  if (isHostWindows.value) await refreshDisplays();
});
</script>

<template>
  <div class="app">
    <header class="cabinet-topbar">
      <div class="cabinet-brand">
        <div class="cabinet-monogram">AJ</div>
        <div class="cabinet-wordmark">
          <span class="wm-tag">CYBER COMPANION · v0.1</span>
          <h1 class="wm-name">AJAZZ <span class="accent">DRIVER</span></h1>
        </div>
      </div>
      <div class="cabinet-meta">
        <span class="crumb">SLOT · <b>{{ devices.length || "—" }}</b></span>
        <span class="sep">│</span>
        <span class="crumb">MODEL · <b>{{ selected?.modelName ?? selected?.product ?? "NONE" }}</b></span>
        <span class="sep">│</span>
        <span class="crumb">FW · <b>{{ info ? `v${info.version}` : "—" }}</b></span>
        <span class="sep">│</span>
        <span class="crumb">RT · <b>{{ info ? `P${info.rtPrecision}` : "—" }}</b></span>
      </div>
      <button class="probe-button" :class="{ 'probe-busy': loading }" :disabled="loading" @click="refresh">
        <span class="probe-led"></span>
        {{ loading ? "Probing…" : "▸ Probe Bus" }}
      </button>
    </header>

    <main class="layout">
      <!-- Sidebar Selector -->
      <section class="sidebar" style="background: rgba(8, 10, 18, 0.7); backdrop-filter: blur(12px);">
        <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px;">
          <span style="font-family: var(--font-mono); font-size: 9px; letter-spacing: 0.22em; color: var(--muted); text-transform: uppercase; font-weight: 700;">
            ▸ Bus · HID Slots
          </span>
          <span class="status-capsule" :class="devices.length ? 'tone-cyan' : 'tone-muted'">
            <span class="led"></span>{{ devices.length }} live
          </span>
        </div>
        <p v-if="!devices.length && !loading" style="padding: 22px 6px; text-align: left; font-size: 11px; color: var(--muted); font-family: var(--font-mono); letter-spacing: 0.05em; line-height: 1.6; border: 1px dashed rgba(255,255,255,0.08); border-radius: 4px;">
          → NO DEVICE ON BUS<br/>
          <span style="opacity: 0.7;">Подключите клавиатуру к USB-порту и нажмите <b>Probe Bus</b>.</span>
        </p>
        <div style="display: flex; flex-direction: column; gap: 8px;">
          <div
            v-for="d in devices"
            :key="d.path"
            class="hid-slot"
            :class="{ 'is-active': selected?.path === d.path }"
            @click="read(d)"
          >
            <span class="slot-pip"></span>
            <div class="slot-body">
              <span class="slot-name">{{ d.modelName ?? d.product ?? "HID Keyboard" }}</span>
              <span class="slot-id">{{ hex(d.vendorId) }}:{{ hex(d.productId) }}</span>
            </div>
          </div>
        </div>

        <nav v-if="info" class="nav-menu" aria-label="Разделы">
          <div class="nav-section">
            <div class="nav-section-label">Устройство</div>
            <button class="nav-item" :class="{ active: activeTab === 'info' }" @click="switchTab('info')">
              <span class="nav-dot" style="color: var(--neon-cyan);"></span>Параметры
            </button>
          </div>
          <div class="nav-section">
            <div class="nav-section-label">Подсветка</div>
            <button class="nav-item" :class="{ active: activeTab === 'lighting' }" :disabled="!ledEffect" @click="switchTab('lighting')">
              <span class="nav-dot" style="color: var(--neon-pink);"></span>Эффекты
            </button>
            <button class="nav-item" :class="{ active: activeTab === 'ambient' }" :disabled="!ledEffect" @click="switchTab('ambient')">
              <span class="nav-dot" style="color: var(--neon-cyan);"></span>Эмбиент-экран
            </button>
            <button class="nav-item" :class="{ active: activeTab === 'equalizer' }" :disabled="!ledEffect" @click="switchTab('equalizer')">
              <span class="nav-dot" style="color: var(--neon-purple);"></span>Эквалайзер
            </button>
            <button class="nav-item" :class="{ active: activeTab === 'pixelart' }" :disabled="!ledEffect" @click="switchTab('pixelart')">
              <span class="nav-dot" style="color: var(--neon-pink);"></span>Pixel-Art GIF
            </button>
          </div>
          <div class="nav-section">
            <div class="nav-section-label">Настройка</div>
            <button class="nav-item" :class="{ active: activeTab === 'magnetic' }" @click="switchTab('magnetic')">
              <span class="nav-dot" style="color: var(--neon-yellow);"></span>Магнитные свичи
            </button>
            <button class="nav-item" :class="{ active: activeTab === 'performance' }" :disabled="!gameMode" @click="switchTab('performance')">
              <span class="nav-dot" style="color: var(--neon-cyan);"></span>Производительность
            </button>
            <button class="nav-item" :class="{ active: activeTab === 'background' }" @click="switchTab('background')">
              <span class="nav-dot" style="color: var(--neon-green);"></span>Фоновый сервис
            </button>
          </div>
          <div class="nav-section">
            <div class="nav-section-label">Сервис</div>
            <button class="nav-item" :class="{ active: activeTab === 'maintenance' }" @click="switchTab('maintenance')">
              <span class="nav-dot" style="color: var(--neon-pink);"></span>Обслуживание
            </button>
          </div>
        </nav>
      </section>

      <!-- Active Content Frame -->
      <section class="detail" style="background: radial-gradient(circle at top right, rgba(255,0,127,0.04), transparent 45%);">
        <div v-if="error" class="banner banner-error" style="border-color: rgba(239, 68, 68, 0.4); box-shadow: 0 0 15px rgba(239,68,68,0.1);">{{ error }}</div>
        <div v-if="successMessage" class="banner banner-success" style="border-color: rgba(16, 185, 129, 0.4); box-shadow: 0 0 15px rgba(16,185,129,0.1);">{{ successMessage }}</div>

        <template v-if="info">
          <!-- Header strip — model callsign + telemetry capsules -->
          <div style="display: flex; justify-content: space-between; align-items: flex-end; gap: 18px; flex-wrap: wrap;">
            <div style="display: flex; flex-direction: column; gap: 4px; min-width: 0;">
              <span style="font-family: var(--font-mono); font-size: 10px; letter-spacing: 0.22em; color: var(--muted); text-transform: uppercase;">
                ▸ Active Device · Slot {{ devices.findIndex(x => x.path === selected?.path) + 1 || "—" }}
              </span>
              <span style="font-family: var(--font); font-size: 26px; font-weight: 800; color: var(--text-bright); letter-spacing: -0.03em;">
                {{ selected?.modelName ?? selected?.product ?? "Tauri Driver" }}
              </span>
              <span style="font-family: var(--font-mono); font-size: 10px; color: var(--muted); letter-spacing: 0.04em; opacity: 0.7; max-width: 540px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                {{ selected?.path }}
              </span>
            </div>

            <div style="display: flex; gap: 8px; flex-wrap: wrap;">
              <span class="status-capsule" :class="info.batteryLevel > 20 ? 'tone-green' : 'tone-pink'">
                <span class="led"></span>BAT · {{ info.batteryLevel.toString().padStart(2,'0') }}%
              </span>
              <span v-if="info.chargeStatus === 1" class="status-capsule tone-yellow">
                <span class="led"></span>Charging
              </span>
              <span class="status-capsule tone-cyan">
                <span class="led"></span>{{ info.workMode === 1 ? "Wired" : info.workMode === 2 ? "2.4 GHz" : "Bluetooth" }}
              </span>
              <span class="status-capsule tone-purple">
                <span class="led"></span>Profile · {{ info.currentProfile }}
              </span>
            </div>
          </div>

          <!-- Core custom content window. flex-shrink: 0 so tall panels (magnetic, lighting)
               keep their natural height and the parent .detail scrolls instead of squashing them. -->
          <div class="card" style="flex: 0 0 auto; background: rgba(12, 14, 25, 0.7); border-color: rgba(255,255,255,0.06); padding: 24px; display: flex; flex-direction: column; gap: 22px;">

            <!-- 1. Technical specs — three-section instrument readout. -->
            <div v-if="activeTab === 'info'" class="instrument-panel">
              <div class="tab-headline">
                <h3>System telemetry</h3>
                <p>▸ DEVICE INFO · cmd 0x10 · 48-byte payload</p>
              </div>

              <div class="section-rule">
                <span class="idx">01</span>
                <span class="title">Identity</span>
                <span class="line"></span>
                <span class="meta">VID:PID locked at probe</span>
              </div>
              <div class="spec-grid">
                <div class="spec-tile accent-pink">
                  <span class="code">▸ MODEL</span>
                  <span class="value">{{ selected?.modelName ?? "GENERIC" }}</span>
                  <span class="footnote">Resolved from models.json</span>
                </div>
                <div class="spec-tile accent-cyan">
                  <span class="code">▸ FIRMWARE</span>
                  <span class="value">v{{ info.version }}</span>
                  <span class="footnote">Frame arch v{{ info.frameVersion }}</span>
                </div>
                <div class="spec-tile">
                  <span class="code">▸ VID:PID</span>
                  <span class="value">{{ hex(info.vendorId) }}<span class="unit">:</span>{{ hex(info.productId) }}</span>
                  <span class="footnote">Sonix VID 0C45</span>
                </div>
                <div class="spec-tile accent-purple">
                  <span class="code">▸ PROFILE</span>
                  <span class="value">P{{ info.currentProfile }}<span class="unit">/4</span></span>
                  <span class="footnote">Onboard slot</span>
                </div>
              </div>

              <div class="section-rule">
                <span class="idx">02</span>
                <span class="title">Hardware</span>
                <span class="line"></span>
                <span class="meta">Flash topology · sensor stack</span>
              </div>
              <div class="spec-grid">
                <div class="spec-tile">
                  <span class="code">▸ ROM FREE</span>
                  <span class="value">{{ info.romSize }}<span class="unit">KB</span></span>
                  <span class="footnote">User region</span>
                </div>
                <div class="spec-tile">
                  <span class="code">▸ MACRO SPACE</span>
                  <span class="value">{{ info.macroSpaceSize }}<span class="unit">B</span></span>
                  <span class="footnote">Sector 0x9800</span>
                </div>
                <div class="spec-tile accent-yellow">
                  <span class="code">▸ SENSOR</span>
                  <span class="value">0x{{ info.sensor.toString(16).toUpperCase().padStart(4, '0') }}</span>
                  <span class="footnote">Hall axis chipset</span>
                </div>
                <div class="spec-tile accent-green">
                  <span class="code">▸ RT PRECISION</span>
                  <span class="value">{{ info.rtPrecision === 0 ? "1/100" : "1/1000" }}</span>
                  <span class="footnote">{{ info.rtPrecision === 0 ? "Standard" : "Esports" }} grade</span>
                </div>
              </div>

              <div class="section-rule">
                <span class="idx">03</span>
                <span class="title">Telemetry</span>
                <span class="line"></span>
                <span class="meta">Live channel state</span>
              </div>
              <div class="spec-grid">
                <div class="spec-tile" :class="info.batteryLevel > 20 ? 'accent-green' : 'accent-pink'">
                  <span class="code">▸ BATTERY</span>
                  <span class="value">{{ info.batteryLevel.toString().padStart(2,'0') }}<span class="unit">%</span></span>
                  <span class="footnote">{{ info.chargeStatus === 1 ? "CHARGING" : "Idle" }}</span>
                </div>
                <div class="spec-tile accent-cyan">
                  <span class="code">▸ LINK</span>
                  <span class="value">{{ info.workMode === 1 ? "USB" : info.workMode === 2 ? "2.4G" : "BLE" }}</span>
                  <span class="footnote">Wire-only commands</span>
                </div>
                <div class="spec-tile">
                  <span class="code">▸ LIGHTING REV</span>
                  <span class="value">v{{ info.lightingVersion }}</span>
                  <span class="footnote">Effect engine</span>
                </div>
                <div class="spec-tile">
                  <span class="code">▸ CHARGE</span>
                  <span class="value">{{ info.chargeStatus === 1 ? "ACT" : "OFF" }}</span>
                  <span class="footnote">Source state</span>
                </div>
              </div>
            </div>

            <!-- 2. Built-in lighting effects -->
            <div v-else-if="activeTab === 'lighting' && ledEffect" class="instrument-panel">
              <div class="tab-headline">
                <h3>Lighting effects</h3>
                <p>▸ FX ENGINE · cmd 0x13 readback · cmd 0x23 write</p>
              </div>

              <div class="section-rule">
                <span class="idx">01</span>
                <span class="title">Effect bank</span>
                <span class="line"></span>
                <span class="meta">Stored at flash 0x9600</span>
              </div>
              <div class="mode-rail">
                <button
                  v-for="mode in lightModes"
                  :key="mode.value"
                  class="mode-tile"
                  :class="{ 'is-on': ledEffect.mode === mode.value }"
                  @click="ledEffect.mode = mode.value; updateStaticBacklight();"
                >
                  <span class="mode-id">M{{ String(mode.value).padStart(2, '0') }}</span>
                  <span class="mode-name">{{ mode.label }}</span>
                </button>
              </div>

              <div class="section-rule">
                <span class="idx">02</span>
                <span class="title">Palette</span>
                <span class="line"></span>
                <span class="meta">Primary · Secondary swatches</span>
              </div>
              <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 12px;">
                <div class="swatch-console">
                  <label class="swatch" :style="{ background: primaryColor, color: primaryColor }">
                    <input type="color" v-model="primaryColor">
                  </label>
                  <div class="swatch-meta">
                    <span class="role">▸ Primary</span>
                    <span class="hex">{{ primaryColor.toUpperCase() }}</span>
                    <span class="rgb">R {{ ledEffect.red.toString().padStart(3,'0') }} · G {{ ledEffect.green.toString().padStart(3,'0') }} · B {{ ledEffect.blue.toString().padStart(3,'0') }}</span>
                  </div>
                  <div class="swatch-channel">
                    <div><span class="label">R</span><div class="bar"><div class="fill" :style="{ width: (ledEffect.red/255*100)+'%', background: '#ff0040' }"></div></div></div>
                    <div><span class="label">G</span><div class="bar"><div class="fill" :style="{ width: (ledEffect.green/255*100)+'%', background: '#39ff14' }"></div></div></div>
                    <div><span class="label">B</span><div class="bar"><div class="fill" :style="{ width: (ledEffect.blue/255*100)+'%', background: '#00f0ff' }"></div></div></div>
                  </div>
                </div>
                <div class="swatch-console">
                  <label class="swatch" :style="{ background: secondaryColor, color: secondaryColor }">
                    <input type="color" v-model="secondaryColor">
                  </label>
                  <div class="swatch-meta">
                    <span class="role">▸ Secondary</span>
                    <span class="hex">{{ secondaryColor.toUpperCase() }}</span>
                    <span class="rgb">R {{ ledEffect.secondaryRed.toString().padStart(3,'0') }} · G {{ ledEffect.secondaryGreen.toString().padStart(3,'0') }} · B {{ ledEffect.secondaryBlue.toString().padStart(3,'0') }}</span>
                  </div>
                  <div class="swatch-channel">
                    <div><span class="label">R</span><div class="bar"><div class="fill" :style="{ width: (ledEffect.secondaryRed/255*100)+'%', background: '#ff0040' }"></div></div></div>
                    <div><span class="label">G</span><div class="bar"><div class="fill" :style="{ width: (ledEffect.secondaryGreen/255*100)+'%', background: '#39ff14' }"></div></div></div>
                    <div><span class="label">B</span><div class="bar"><div class="fill" :style="{ width: (ledEffect.secondaryBlue/255*100)+'%', background: '#00f0ff' }"></div></div></div>
                  </div>
                </div>
              </div>

              <div class="section-rule">
                <span class="idx">03</span>
                <span class="title">Modulation</span>
                <span class="line"></span>
                <span class="meta">Brightness · Speed · Direction</span>
              </div>
              <div style="display: flex; flex-direction: column; gap: 6px;">
                <div class="field-strip">
                  <span class="field-label">▸ Brightness</span>
                  <span class="field-control"><input type="range" min="0" max="5" v-model.number="ledEffect.brightness" class="range-slider range-slider-cyan" style="flex:1;"></span>
                  <span class="field-readout">{{ (ledEffect.brightness * 20).toString().padStart(3,'0') }}<span class="unit">%</span></span>
                </div>
                <div class="field-strip">
                  <span class="field-label">▸ Wave Speed</span>
                  <span class="field-control"><input type="range" min="1" max="5" v-model.number="ledEffect.speed" class="range-slider range-slider-cyan" style="flex:1;"></span>
                  <span class="field-readout">{{ (ledEffect.speed * 20).toString().padStart(3,'0') }}<span class="unit">%</span></span>
                </div>
                <div class="field-strip">
                  <span class="field-label">▸ Direction</span>
                  <span class="field-control" style="display: flex; gap: 6px;">
                    <div class="toggle-bank">
                      <button :class="{ 'is-on': ledEffect.direction === 0 }" @click="ledEffect.direction = 0">→ R</button>
                      <button :class="{ 'is-on': ledEffect.direction === 1 }" @click="ledEffect.direction = 1">← L</button>
                      <button :class="{ 'is-on': ledEffect.direction === 2 }" @click="ledEffect.direction = 2">↓ D</button>
                      <button :class="{ 'is-on': ledEffect.direction === 3 }" @click="ledEffect.direction = 3">↑ U</button>
                    </div>
                  </span>
                  <span class="field-readout">D{{ ledEffect.direction }}</span>
                </div>
              </div>

              <div style="display: flex; justify-content: flex-end; gap: 10px; border-top: 1px dashed rgba(255,255,255,0.06); padding-top: 16px;">
                <button class="console-action tone-cyan" :disabled="applying" @click="applyLedEffect">
                  {{ applying ? "Writing…" : "Write Effect" }}
                </button>
              </div>
            </div>

            <!-- 3. Screen Sync Mirror / Ambient Sync -->
            <div v-else-if="activeTab === 'ambient'" class="instrument-panel">
              <div class="tab-headline">
                <h3>Ambient screen mirror</h3>
                <p>▸ getDisplayMedia → 16×6 colour sample → cmd 0x32 stream</p>
              </div>

              <div class="section-rule">
                <span class="idx">01</span>
                <span class="title">Source</span>
                <span class="line"></span>
                <span class="meta">{{ isAmbientActive ? 'streaming · live ~25-30 fps' : 'idle · awaiting capture grant' }}</span>
              </div>

              <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 14px; align-items: stretch;">
                <div style="display: flex; flex-direction: column; gap: 12px;">
                  <div class="status-capsule" :class="isAmbientActive ? 'tone-green' : 'tone-muted'" style="align-self: flex-start;">
                    <span class="led"></span>{{ isAmbientActive ? 'STREAMING' : 'STAND-BY' }}
                  </div>
                  <p style="margin: 0; font-size: 12px; color: var(--muted); line-height: 1.6; font-family: var(--font-mono); letter-spacing: 0.02em;">
                    Браузер запросит экран / окно / вкладку — лучше выбрать конкретное окно: меньше шум, точнее средний цвет под каждым физическим LED.
                  </p>
                  <div style="margin-top: auto;">
                    <button v-if="!isAmbientActive" class="console-action tone-cyan" @click="startAmbientSync" :disabled="!selected">
                      Engage Mirror
                    </button>
                    <button v-else class="console-action tone-danger" @click="stopAmbientSync">
                      Halt Stream
                    </button>
                  </div>
                </div>

                <div style="position: relative; border: 1px solid rgba(0,240,255,0.25); border-radius: 4px; overflow: hidden;
                            background: linear-gradient(135deg, rgba(0,240,255,0.04), rgba(0,0,0,0.6));
                            min-height: 180px; display: flex; align-items: center; justify-content: center;">
                  <div v-if="!isAmbientActive" style="display: flex; flex-direction: column; align-items: center; gap: 6px; opacity: 0.5;">
                    <svg width="42" height="42" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="color: var(--neon-cyan);">
                      <rect x="2" y="3" width="20" height="14" rx="1" ry="1"/>
                      <line x1="8" y1="21" x2="16" y2="21"/>
                      <line x1="12" y1="17" x2="12" y2="21"/>
                    </svg>
                    <span style="font-family: var(--font-mono); font-size: 10px; letter-spacing: 0.2em; color: var(--muted); text-transform: uppercase;">No signal</span>
                  </div>
                  <div v-else style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                    <span style="font-family: var(--font-mono); font-size: 10px; letter-spacing: 0.2em; color: var(--neon-cyan); text-transform: uppercase; text-shadow: 0 0 12px rgba(0,240,255,0.5);">▸ Live capture</span>
                    <span class="numeric-readout">~30<span class="unit">FPS</span></span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 3b. Background service (helper daemon, runs without the app open) -->
            <div v-else-if="activeTab === 'background'" class="instrument-panel">
              <div class="tab-headline">
                <h3>Background service</h3>
                <p>▸ ajazz-helperd · DXGI screen + GIF loop · HKCU autostart</p>
              </div>

              <div class="compat-banner" :class="{ 'tone-warn': !isHostWindows }">
                <span class="badge">{{ isHostWindows ? "WIN32 · READY" : "HOST · " + (isHostWindows ? "WIN" : "UNIX") }}</span>
                <span style="font-family: var(--font-mono); letter-spacing: 0.04em;">
                  {{ isHostWindows
                     ? "Screen capture via DXGI Desktop Duplication. Autostart via HKCU\\…\\Run."
                     : "Хост — не Windows: сервис не запустится. Только UI-вкладка для предпросмотра." }}
                </span>
              </div>

              <div class="section-rule">
                <span class="idx">01</span>
                <span class="title">Mode select</span>
                <span class="line"></span>
                <span class="meta">writes to %APPDATA%/ajazz-driver/helper.json</span>
              </div>

              <div class="toggle-bank tone-green" style="display: flex; width: max-content;">
                <button :class="{ 'is-on': helperMode === 'off' }" :disabled="!isHostWindows" @click="setHelperMode('off')">Off</button>
                <button :class="{ 'is-on': helperMode === 'screen' }" :disabled="!isHostWindows" @click="setHelperMode('screen')">Screen</button>
                <button :class="{ 'is-on': helperMode === 'gif' }" :disabled="!isHostWindows" @click="setHelperMode('gif')">GIF</button>
              </div>

              <!-- Screen selector — only relevant when helper is in screen-mirror mode -->
              <template v-if="helperMode === 'screen'">
                <div class="section-rule">
                  <span class="idx">02</span>
                  <span class="title">Display</span>
                  <span class="line"></span>
                  <span class="meta">{{ helperDisplays.length }} monitor{{ helperDisplays.length === 1 ? '' : 's' }} detected</span>
                </div>
                <div class="field-strip">
                  <span class="field-label">▸ Mirror source</span>
                  <span class="field-control" style="display: flex; gap: 6px; flex-wrap: wrap;">
                    <select
                      v-model.number="helperScreenIndex"
                      :disabled="!isHostWindows || !helperDisplays.length"
                      @change="pushHelperConfig"
                      class="select-input"
                      style="flex: 1; min-width: 220px; background: rgba(0,0,0,0.5);"
                    >
                      <option v-for="d in helperDisplays" :key="d.index" :value="d.index">
                        Display {{ d.index + 1 }} · {{ d.width }}×{{ d.height }}{{ d.isPrimary ? ' (primary)' : '' }}
                      </option>
                    </select>
                    <button class="console-action tone-cyan" :disabled="!isHostWindows" @click="refreshDisplays" style="flex: 0 0 auto;">
                      ↺ Refresh
                    </button>
                  </span>
                </div>
              </template>

              <div class="section-rule">
                <span class="idx">{{ helperMode === 'screen' ? '03' : '02' }}</span>
                <span class="title">GIF source</span>
                <span class="line"></span>
                <span class="meta">{{ helperGifPath ? 'asset bound' : 'no asset' }}</span>
              </div>

              <div class="asset-console">
                <div class="asset-meta" style="min-width: 0;">
                  <span class="kicker">▸ File</span>
                  <span style="color: var(--text-bright); font-family: var(--font-mono); font-size: 13px; word-break: break-all; overflow-wrap: anywhere;">
                    {{ helperGifPath || "— ничего не выбрано —" }}
                  </span>
                  <span class="stats">{{ helperGifPath ? 'PATH SET · helper polls every tick' : 'NO ASSET LOADED' }}</span>
                </div>
                <button class="console-action tone-pink" :disabled="!isHostWindows" @click="pickHelperGif" style="flex: 0 0 auto;">
                  ▸ Pick GIF…
                </button>
              </div>

              <div class="section-rule">
                <span class="idx">03</span>
                <span class="title">Frame rate</span>
                <span class="line"></span>
                <span class="meta">5 ≤ fps ≤ 60</span>
              </div>

              <div class="field-strip">
                <span class="field-label">▸ FPS Target</span>
                <span class="field-control">
                  <input type="range" min="5" max="60" v-model.number="helperFps" :disabled="!isHostWindows" class="range-slider range-slider-cyan" style="flex: 1;">
                </span>
                <span class="field-readout">{{ helperFps.toString().padStart(2, '0') }}<span class="unit">FPS</span></span>
              </div>

              <div class="section-rule">
                <span class="idx">04</span>
                <span class="title">Autostart</span>
                <span class="line"></span>
                <span class="meta">HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run</span>
              </div>

              <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                <button class="console-action tone-green" :disabled="!isHostWindows" @click="enableAutostart">
                  Install Autostart
                </button>
                <button class="console-action tone-pink" :disabled="!isHostWindows" @click="disableAutostart">
                  Remove Autostart
                </button>
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

                  <!-- Scale Mode Selection -->
                  <div style="display: flex; flex-direction: column; gap: 6px; margin-top: 4px;">
                    <span style="font-size: 11px; font-weight: 700; color: rgba(255,255,255,0.4); text-transform: uppercase; letter-spacing: 0.05em; text-align: left;">Режим масштабирования:</span>
                    <div style="display: flex; gap: 4px; background: rgba(0,0,0,0.4); padding: 4px; border-radius: 8px; border: 1px solid rgba(255,0,127,0.15);">
                      <button 
                        @click="gifScaleMode = 'cover'"
                        style="flex: 1; font-size: 11px; padding: 6px; font-weight: 700; border: none; border-radius: 6px; cursor: pointer; transition: all 0.2s ease; outline: none;"
                        :style="gifScaleMode === 'cover' ? { background: 'var(--neon-pink)', color: '#fff', boxShadow: '0 0 10px rgba(255,0,127,0.4)' } : { background: 'transparent', color: 'rgba(255,255,255,0.6)' }"
                      >
                        С обрезкой (Cover)
                      </button>
                      <button 
                        @click="gifScaleMode = 'contain'"
                        style="flex: 1; font-size: 11px; padding: 6px; font-weight: 700; border: none; border-radius: 6px; cursor: pointer; transition: all 0.2s ease; outline: none;"
                        :style="gifScaleMode === 'contain' ? { background: 'var(--neon-pink)', color: '#fff', boxShadow: '0 0 10px rgba(255,0,127,0.4)' } : { background: 'transparent', color: 'rgba(255,255,255,0.6)' }"
                      >
                        С полями (Contain)
                      </button>
                    </div>
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

            <!-- 5b. Magnetic switches: instrument panel — calibration, presets, RT, SOCD, scope. -->
            <div v-else-if="activeTab === 'magnetic'" class="instrument-panel">

              <!-- Header: device callsign + status pips -->
              <div class="panel-status-strip">
                <div class="panel-status-cell">
                  <span class="led-pip" :style="{ color: selected ? 'var(--neon-cyan)' : 'var(--muted)' }"
                        :class="{ dim: !selected }"></span>
                  <div style="display: flex; flex-direction: column;">
                    <span class="label">Channel</span>
                    <span class="value">{{ selected?.modelName ?? selected?.product ?? "—" }}</span>
                  </div>
                </div>
                <div class="panel-status-cell">
                  <span class="led-pip"
                        :style="{ color: calibrationActive ? 'var(--neon-green)' : 'var(--muted)' }"
                        :class="{ dim: !calibrationActive, pulsing: calibrationActive }"></span>
                  <div style="display: flex; flex-direction: column;">
                    <span class="label">Calibrate</span>
                    <span class="value">{{ calibrationActive ? `${calibratedCount}/${keysList.length}` : "READY" }}</span>
                  </div>
                </div>
                <div class="panel-status-cell">
                  <span class="led-pip"
                        :style="{ color: activePresetId ? rtPresets.find(p => p.id === activePresetId)?.accent : 'var(--muted)' }"
                        :class="{ dim: !activePresetId }"></span>
                  <div style="display: flex; flex-direction: column;">
                    <span class="label">Profile</span>
                    <span class="value">{{ activePresetId ? rtPresets.find(p => p.id === activePresetId)?.name : "CUSTOM" }}</span>
                  </div>
                </div>
                <div class="panel-status-cell">
                  <span class="led-pip"
                        :style="{ color: adcMonitorActive ? 'var(--neon-yellow)' : 'var(--muted)' }"
                        :class="{ dim: !adcMonitorActive, pulsing: adcMonitorActive }"></span>
                  <div style="display: flex; flex-direction: column;">
                    <span class="label">Scope</span>
                    <span class="value">{{ adcMonitorActive ? `${activeScopeChannels.length}CH` : "OFF" }}</span>
                  </div>
                </div>
              </div>

              <!-- ─── 01 · CALIBRATION ────────────────────────────────────────────────── -->
              <div class="section-rule">
                <span class="idx">01</span>
                <span class="title">Travel calibration</span>
                <span class="line"></span>
                <span class="meta">cmd 0x69 · streaming notify 0xFB</span>
              </div>

              <div style="display: flex; flex-direction: column; gap: 14px;">
                <div style="display: flex; align-items: center; justify-content: space-between; gap: 12px; flex-wrap: wrap;">
                  <p style="margin: 0; font-size: 12px; color: var(--muted); line-height: 1.5; max-width: 520px;">
                    Нажмите каждую клавишу до упора — ход датчика Холла запомнится и калибровка станет
                    устойчивой к температурному дрейфу. Зелёный — готово, жёлтый — есть касание, розовый — ждём.
                  </p>
                  <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                    <button v-if="!calibrationActive" class="btn" :disabled="!selected" @click="startCalibration"
                            style="background: var(--neon-cyan); color: #000;">
                      ▸ Начать
                    </button>
                    <template v-else>
                      <button class="btn" @click="finishCalibration" style="background: var(--neon-green); color: #000;">
                        ✓ Сохранить
                      </button>
                      <button class="btn btn-secondary" @click="cancelCalibration">✕ Отмена</button>
                    </template>
                  </div>
                </div>

                <div v-if="calibrationActive" class="flex-center" style="margin-top: 10px;">
                  <div class="relative p-2 rounded-2.5 bg-primary:10 w-fit">
                    <div class="rounded-sm w-fit relative" style="padding: 2px; --kb-unit: 20px; --kb-gap: 2px;">
                      <div 
                        v-for="(row, ri) in rawKeyboardList" 
                        :key="ri" 
                        class="relative flex justify-between gap-1 mb-1 last:mb-0"
                      >
                        <div
                          v-for="key in row"
                          :key="key.value"
                          :class="['key-item', 'w-14', 'h-14', key.className]"
                          :style="{ background: calibrationColor(key.value), borderColor: 'rgba(255,255,255,0.06)' }"
                        >
                          <span style="font-size: 6px; opacity: 0.8; font-weight: 700;">{{ key.name }}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- ─── 02 · RESPONSE PRESETS ───────────────────────────────────────────── -->
              <div class="section-rule">
                <span class="idx">02</span>
                <span class="title">Response presets</span>
                <span class="line"></span>
                <span class="meta">one-click → cmd 0x27 write</span>
              </div>

              <div class="preset-rail">
                <button
                  v-for="preset in rtPresets"
                  :key="preset.id"
                  class="preset-chip"
                  :class="{ 'is-active': activePresetId === preset.id }"
                  :disabled="!selected || applying"
                  @click="applyPreset(preset.id)"
                >
                  <span class="preset-corner">{{ preset.label }}</span>
                  <span class="preset-name" :style="{ color: activePresetId === preset.id ? 'var(--neon-cyan)' : preset.accent }">
                    {{ preset.name }}
                  </span>
                  <svg class="preset-curve" viewBox="0 0 100 40" preserveAspectRatio="none">
                    <path :d="`${presetCurvePath(preset.curve)} L 100,40 L 0,40 Z`" :fill="preset.accent" fill-opacity="0.18" />
                    <path :d="presetCurvePath(preset.curve)" fill="none" :stroke="preset.accent" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                  <span class="preset-strapline">{{ preset.desc }}</span>
                </button>
              </div>

              <!-- ─── 03 · RAPID TRIGGER (manual override) ────────────────────────────── -->
              <div class="section-rule">
                <span class="idx">03</span>
                <span class="title">Rapid trigger · global</span>
                <span class="line"></span>
                <span class="meta">{{ magneticKeys ? `${magneticKeys.length} slots resident` : "no payload loaded" }}</span>
              </div>

              <div style="display: flex; flex-direction: column; gap: 16px;">
                <div style="display: flex; align-items: center; justify-content: space-between; gap: 12px; flex-wrap: wrap;">
                  <p style="margin: 0; font-size: 12px; color: var(--muted); line-height: 1.5; max-width: 520px;">
                    Тонкая настройка профиля — для тех, кто хочет своё. Сначала «Прочитать с клавиатуры»,
                    чтобы получить текущие значения, потом крутите.
                  </p>
                  <button class="btn btn-secondary" :disabled="magneticLoading || !selected" @click="loadMagneticAxis">
                    {{ magneticLoading ? "↺ Чтение…" : "↺ Прочитать с клавиатуры" }}
                  </button>
                </div>

                <template v-if="magneticKeys">
                  <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 16px;">
                    <div class="form-group">
                      <label style="display: flex; justify-content: space-between; align-items: center;">
                        <span>Trigger stroke</span>
                        <span class="numeric-readout">{{ rtGlobal.triggerKeyStroke.toFixed(2) }} <span class="unit">MM</span></span>
                      </label>
                      <input type="range" min="0.1" max="4.0" step="0.05" v-model.number="rtGlobal.triggerKeyStroke" class="range-slider" @input="activePresetId = null">
                    </div>
                    <div class="form-group">
                      <label style="display: flex; justify-content: space-between; align-items: center;">
                        <span>Press RT</span>
                        <span class="numeric-readout">{{ rtGlobal.pressRT.toFixed(2) }} <span class="unit">MM</span></span>
                      </label>
                      <input type="range" min="0.01" max="2.0" step="0.01" v-model.number="rtGlobal.pressRT" class="range-slider" @input="activePresetId = null">
                    </div>
                    <div class="form-group">
                      <label style="display: flex; justify-content: space-between; align-items: center;">
                        <span>Release RT</span>
                        <span class="numeric-readout">{{ rtGlobal.releaseRT.toFixed(2) }} <span class="unit">MM</span></span>
                      </label>
                      <input type="range" min="0.01" max="2.0" step="0.01" v-model.number="rtGlobal.releaseRT" class="range-slider" @input="activePresetId = null">
                    </div>
                  </div>

                  <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 10px;">
                    <div class="switch-container" style="background: rgba(0,0,0,0.25);">
                      <div class="switch-info">
                        <span class="switch-title" style="font-size: 13px;">Whole-fast</span>
                        <span class="switch-desc">Динамическая точка срабатывания — адаптируется на лету.</span>
                      </div>
                      <label class="switch">
                        <input type="checkbox" v-model="rtGlobal.isWholeFast" @change="activePresetId = null">
                        <span class="slider"></span>
                      </label>
                    </div>
                    <div class="switch-container" style="background: rgba(0,0,0,0.25);">
                      <div class="switch-info">
                        <span class="switch-title" style="font-size: 13px;">Rampage Mode</span>
                        <span class="switch-desc">Экстремальная чувствительность. Только для качественных свичей.</span>
                      </div>
                      <label class="switch">
                        <input type="checkbox" v-model="rtGlobal.isRampageMode" @change="activePresetId = null">
                        <span class="slider"></span>
                      </label>
                    </div>
                  </div>

                  <div style="display: flex; gap: 10px; flex-wrap: wrap; align-items: center; padding-top: 10px; border-top: 1px dashed rgba(255,255,255,0.06);">
                    <button class="btn" :disabled="applying" @click="applyRapidTriggerToAll"
                            style="background: var(--neon-purple); color: #fff;">
                      {{ applying ? "→ Запись…" : "→ Записать в железо" }}
                    </button>
                    <button class="btn btn-secondary" :disabled="applying" @click="triggerReset(5)">
                      Сбросить калибровку
                    </button>
                  </div>
                </template>
                <p v-else-if="!magneticLoading" style="font-size: 12px; color: var(--muted); margin: 0;">
                  → Нажмите «Прочитать с клавиатуры» чтобы получить текущие 128 слотов.
                </p>
              </div>

              <!-- ─── 04 · SOCD MATRIX ────────────────────────────────────────────────── -->
              <div class="section-rule">
                <span class="idx">04</span>
                <span class="title">SOCD matrix</span>
                <span class="line"></span>
                <span class="meta">client-side preview · device write pending firmware reverse</span>
              </div>

              <div class="socd-builder">
                <!-- Pair builder canvas -->
                <div class="socd-canvas">
                  <div class="socd-keys">
                    <label class="socd-key" :class="socdK1 != null ? 'is-k1' : 'is-empty'">
                      <span class="kicker">Key 1</span>
                      <span class="cap">{{ socdK1 != null ? (keyLabelByValue[socdK1] || `#${socdK1}`) : "—" }}</span>
                      <select v-model.number="socdK1">
                        <option :value="null">— выбрать —</option>
                        <option v-for="k in keysList" :key="`k1-${k.idx}`" :value="k.idx">{{ k.label }}</option>
                      </select>
                    </label>

                    <svg class="socd-bridge" viewBox="0 0 70 60" fill="none">
                      <path d="M 4 30 C 20 4, 50 4, 66 30" stroke="#9d00ff" stroke-width="1.5" fill="none" stroke-dasharray="3 3" opacity="0.8"/>
                      <circle cx="4" cy="30" r="3" fill="#ff007f"/>
                      <circle cx="66" cy="30" r="3" fill="#00f0ff"/>
                      <text x="35" y="50" text-anchor="middle" font-family="JetBrains Mono, monospace" font-size="7" fill="#8295a5" letter-spacing="1.5">SOCD</text>
                    </svg>

                    <label class="socd-key" :class="socdK2 != null ? 'is-k2' : 'is-empty'">
                      <span class="kicker">Key 2</span>
                      <span class="cap">{{ socdK2 != null ? (keyLabelByValue[socdK2] || `#${socdK2}`) : "—" }}</span>
                      <select v-model.number="socdK2">
                        <option :value="null">— выбрать —</option>
                        <option v-for="k in keysList" :key="`k2-${k.idx}`" :value="k.idx">{{ k.label }}</option>
                      </select>
                    </label>
                  </div>

                  <div class="socd-mode-strip">
                    <button v-for="m in SOCD_MODES" :key="m.id" class="socd-mode-pill"
                            :class="{ 'is-active': socdMode === m.id }"
                            @click="socdMode = m.id">
                      <span class="name">{{ m.name }}</span>
                      <span class="desc">{{ m.desc }}</span>
                    </button>
                  </div>

                  <div style="display: flex; align-items: center; justify-content: space-between; gap: 10px; flex-wrap: wrap;">
                    <button class="btn" :disabled="socdK1 == null || socdK2 == null"
                            @click="addSocdPair"
                            style="background: var(--neon-purple); color: #fff;">
                      + Добавить пару
                    </button>
                    <div class="socd-template-row">
                      <span style="font-family: var(--font-mono); font-size: 9px; color: var(--muted); letter-spacing: 0.15em; align-self: center;">QUICK:</span>
                      <button class="tpl" @click="applySocdTemplate('wasd')">WASD</button>
                      <button class="tpl" @click="applySocdTemplate('arrows')">ARROWS</button>
                    </div>
                  </div>
                </div>

                <!-- Active pairs roster -->
                <div style="display: flex; flex-direction: column; gap: 10px;">
                  <div style="display: flex; align-items: center; justify-content: space-between;">
                    <span style="font-family: var(--font-mono); font-size: 10px; letter-spacing: 0.18em; color: var(--muted); text-transform: uppercase;">
                      Active pairs · {{ socdPairs.length }}
                    </span>
                    <button v-if="socdPairs.length" class="btn btn-secondary" style="padding: 4px 10px; font-size: 11px;"
                            @click="socdPairs = []; persistSocd();">
                      ✕ Очистить все
                    </button>
                  </div>
                  <div v-if="!socdPairs.length"
                       style="padding: 16px; border: 1px dashed rgba(255,255,255,0.1); border-radius: 4px; text-align: center;
                              font-family: var(--font-mono); font-size: 11px; color: var(--muted); letter-spacing: 0.1em;">
                    NO PAIRS CONFIGURED
                  </div>
                  <div v-else class="socd-list">
                    <div v-for="p in socdPairs" :key="p.id" class="socd-row">
                      <span class="k k1">{{ keyLabelByValue[p.k1] || `#${p.k1}` }}</span>
                      <span class="bridge">↔</span>
                      <span class="k k2">{{ keyLabelByValue[p.k2] || `#${p.k2}` }}</span>
                      <span class="mode-tag">{{ SOCD_MODES.find(m => m.id === p.mode)?.name || p.mode }}</span>
                      <button class="remove" @click="removeSocdPair(p.id)" title="Удалить пару">×</button>
                    </div>
                  </div>
                </div>
              </div>

              <!-- ─── 05 · SCOPE — LIVE HALL-EFFECT ADC ───────────────────────────────── -->
              <div class="section-rule">
                <span class="idx">05</span>
                <span class="title">Scope · live ADC</span>
                <span class="line"></span>
                <span class="meta">cmd 0xFB sample stream · {{ activeScopeChannels.length }} CH active</span>
              </div>

              <div class="scope-frame">
                <div class="scope-bezel">
                  <div class="knob-cluster">
                    <span class="knob">▸ SRC: HALL-Σ</span>
                    <span class="knob">▸ T/DIV: 40MS</span>
                    <span class="knob">▸ V/DIV: 0.5MM</span>
                  </div>
                  <button class="arm-toggle" :class="{ armed: adcMonitorActive }"
                          :disabled="!selected" @click="toggleAdcMonitor">
                    {{ adcMonitorActive ? "● ARMED" : "○ ARM" }}
                  </button>
                </div>
                <div class="scope-screen">
                  <svg v-if="activeScopeChannels.length" class="scope-canvas" viewBox="0 0 100 100"
                       preserveAspectRatio="none">
                    <path v-for="(idx, pos) in activeScopeChannels" :key="idx"
                          :d="scopeTracePath(adcTraces[idx])"
                          :stroke="scopeColor(idx, pos)"
                          :style="{ filter: `drop-shadow(0 0 2px ${scopeColor(idx, pos)})` }"
                          fill="none" stroke-width="0.6" stroke-linecap="round" stroke-linejoin="round" />
                  </svg>
                  <div v-else class="scope-empty">
                    <span class="blink">▸ ARM SCOPE TO BEGIN ◂</span>
                    <span style="opacity: 0.5; font-size: 9px;">no samples — press a key after arming</span>
                  </div>
                </div>
                <div v-if="activeScopeChannels.length" class="scope-channels">
                  <div v-for="(idx, pos) in activeScopeChannels" :key="idx" class="scope-channel"
                       :style="{ color: scopeColor(idx, pos) }">
                    <span class="swatch"></span>
                    <span class="label">{{ keyLabelByValue[idx] || `KEY ${idx.toString(16).padStart(2,'0')}` }}</span>
                    <span class="reading">{{ ((adcTraces[idx]?.[adcTraces[idx].length-1] ?? 0) * 100).toFixed(0).padStart(2, '0') }}%</span>
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
              
              <div class="flex-center mt-20">
                <div class="relative p-4 rounded-2.5 bg-primary:10 w-fit">
                  <div class="p-1 rounded-sm w-fit relative" style="--kb-unit: 22px; --kb-gap: 3px;">
                    <div 
                      v-for="(row, rIdx) in rawKeyboardList" 
                      :key="rIdx" 
                      class="relative flex justify-between gap-1 mb-1 last:mb-0"
                    >
                      <div
                        v-for="key in row"
                        :key="key.value"
                        :class="['key-item', 'w-14', 'h-14', key.className]"
                        :style="{ 
                          borderColor: liveKeyColors[key.value] ? `${liveKeyColors[key.value]}` : 'rgba(255,255,255,0.08)',
                          boxShadow: liveKeyColors[key.value] ? `0 0 12px ${liveKeyColors[key.value]}33` : 'none',
                          color: liveKeyColors[key.value] ? '#fff' : '#a9b1d6',
                          backgroundColor: liveKeyColors[key.value] ? `${liveKeyColors[key.value]}22` : 'rgb(28, 28, 28)'
                        }"
                      >
                        <div class="key-highlight-top"></div>
                        <div class="key-content">
                          <span class="key-label" style="font-size: 6px; font-weight: 700;">{{ key.name }}</span>
                        </div>
                        <div class="key-shadow-bottom"></div>
                        <!-- Glowing custom LED projection -->
                        <div 
                          class="key-cap-led" 
                          :style="{ 
                            backgroundColor: liveKeyColors[key.value] || 'transparent',
                            boxShadow: liveKeyColors[key.value] ? `0 -1px 12px 2px ${liveKeyColors[key.value]}` : 'none'
                          }"
                        ></div>
                      </div>
                    </div>
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
