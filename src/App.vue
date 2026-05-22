<script setup lang="ts">
import { ref, onMounted } from "vue";
import { listDevices, getDeviceInfo, type DeviceSummary, type DeviceInfo } from "./lib/api";

const devices = ref<DeviceSummary[]>([]);
const selected = ref<DeviceSummary | null>(null);
const info = ref<DeviceInfo | null>(null);
const error = ref<string | null>(null);
const loading = ref(false);

async function refresh() {
  error.value = null;
  loading.value = true;
  try {
    devices.value = await listDevices();
    if (devices.value.length && !selected.value) selected.value = devices.value[0];
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
  loading.value = true;
  try {
    info.value = await getDeviceInfo(dev.path);
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

const hex = (n: number) => "0x" + n.toString(16).padStart(4, "0");

onMounted(refresh);
</script>

<template>
  <div class="app">
    <header class="topbar">
      <h1>AJAZZ Driver</h1>
      <button class="btn" :disabled="loading" @click="refresh">
        {{ loading ? "…" : "Обновить" }}
      </button>
    </header>

    <!-- Responsive: sidebar + detail collapse to a single column on narrow windows. -->
    <main class="layout">
      <section class="sidebar">
        <h2>Устройства</h2>
        <p v-if="!devices.length && !loading" class="muted">
          Клавиатура не найдена. Подключите по USB и нажмите «Обновить».
        </p>
        <ul class="device-list">
          <li
            v-for="d in devices"
            :key="d.path"
            :class="{ active: selected?.path === d.path }"
            @click="read(d)"
          >
            <span class="name">{{ d.modelName ?? d.product ?? "HID-устройство" }}</span>
            <span class="ids">{{ hex(d.vendorId) }}:{{ hex(d.productId) }}</span>
          </li>
        </ul>
      </section>

      <section class="detail">
        <p v-if="error" class="error">{{ error }}</p>
        <template v-if="info">
          <h2>{{ selected?.modelName ?? selected?.product ?? "Устройство" }}</h2>
          <div class="grid">
            <div class="cell"><label>Прошивка</label><b>{{ info.version }}</b></div>
            <div class="cell"><label>VID:PID</label><b>{{ hex(info.vendorId) }}:{{ hex(info.productId) }}</b></div>
            <div class="cell"><label>Батарея</label><b>{{ info.batteryLevel }}%</b></div>
            <div class="cell"><label>Профиль</label><b>{{ info.currentProfile }}</b></div>
            <div class="cell"><label>Режим</label><b>{{ info.workMode }}</b></div>
            <div class="cell"><label>RT precision</label><b>{{ info.rtPrecision }}</b></div>
            <div class="cell"><label>Frame ver.</label><b>{{ info.frameVersion }}</b></div>
            <div class="cell"><label>Lighting ver.</label><b>{{ info.lightingVersion }}</b></div>
          </div>
        </template>
        <p v-else-if="!error" class="muted">Выберите устройство, чтобы прочитать его данные.</p>
      </section>
    </main>
  </div>
</template>
