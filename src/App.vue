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
  type DeviceSummary, 
  type DeviceInfo,
  type GameMode,
  type LedEffect,
  type LedColor
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

// Effect catalog confirmed against AK980 MAX hardware (the device uses the SECOND lighting
// catalog from the upstream bundle, not the first): mode 1 = flowing rainbow, 2 = static,
// 5 = off, 128 = custom per-key buffer. The previous list was the wrong catalog (Starry/Snow/…),
// which is why "static" actually showed a rolling rainbow.
const lightModes = [
  { value: 1, label: "Радуга / течение (Flowing)" },
  { value: 2, label: "Статичный (Static)" },
  { value: 3, label: "Дыхание (Breathing)" },
  { value: 4, label: "Спектр (Spectrum)" },
  { value: 5, label: "Выключена (Off)" },
  { value: 6, label: "Динамика (Dynamic)" },
  { value: 7, label: "Цепная реакция (Chain)" },
  { value: 8, label: "Звук 1 (Music 1)" },
  { value: 9, label: "Звук 2 (Music 2)" },
  { value: 10, label: "Звук 3 (Music 3)" },
  { value: 11, label: "Звук 4 (Music 4)" },
  { value: 128, label: "Своя подсветка (Custom)" }
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
                  v-for="(row, rIdx) in rawKeyboardList" 
                  :key="rIdx" 
                  class="keyboard-row"
                >
                  <div
                    v-for="key in row"
                    :key="key.value"
                    :class="['key-item', key.className]"
                    :style="{ 
                      borderColor: liveKeyColors[key.value] ? `${liveKeyColors[key.value]}` : 'rgba(255,255,255,0.08)',
                      boxShadow: liveKeyColors[key.value] ? `0 0 12px ${liveKeyColors[key.value]}33` : 'none',
                      color: liveKeyColors[key.value] ? '#fff' : '#a9b1d6'
                    }"
                  >
                    <div class="key-highlight-top"></div>
                    <div class="key-content">
                      <span class="key-label">{{ key.name }}</span>
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
