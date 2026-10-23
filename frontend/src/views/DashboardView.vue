<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import SensorGauge from '../components/SensorGauge.vue'
import DefectFeed, { type Defect } from '../components/DefectFeed.vue'

// Reactive state for WebSocket streaming
const vibration = ref(120.0)
const temperature = ref(65.0)
const isConnected = ref(false)
const defects = ref<Defect[]>([])

let socket: WebSocket | null = null

// ============================================================================
// HOT PATH: WebSocket Connection Lifecycle
// Stream sub-second telemetry updates directly from the Rust Axum backend.
// ============================================================================
const connectWebSocket = () => {
  socket = new WebSocket('ws://127.0.0.1:3000/ws/telemetry')

  socket.onopen = () => {
    isConnected.value = true
  }

  socket.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data)
      vibration.value = data.vibration || vibration.value
      temperature.value = data.temperature || temperature.value
    } catch (e) {
      console.warn('Failed to parse telemetry packet:', e)
    }
  }

  // SAD PATH: Auto-reconnect handling on server drop
  socket.onclose = () => {
    isConnected.value = false
    setTimeout(connectWebSocket, 3000)
  }
}

// On-demand API trigger calling Rust ONNX vision inspection endpoint
const triggerInspection = async () => {
  try {
    const res = await fetch('http://127.0.0.1:3000/api/v1/inspect', {
      method: 'POST',
      body: new Uint8Array([1, 2, 3, 4]), // Dummy frame buffer payload
    })
    const data = await res.json()
    if (data.detection) {
      defects.value.unshift({
        id: crypto.randomUUID(),
        defectType: data.detection.defect_type,
        confidence: data.detection.confidence,
        timestamp: new Date().toLocaleTimeString(),
      })
    }
  } catch (err) {
    console.error('Inspection API request failed:', err)
  }
}

onMounted(() => {
  connectWebSocket()
})

onUnmounted(() => {
  socket?.close()
})
</script>

<template>
  <div class="min-h-screen bg-slate-950 text-slate-100 p-6 md:p-10 font-sans">
    <!-- Header bar -->
    <header class="flex items-center justify-between border-b border-slate-800 pb-6 mb-8">
      <div>
        <h1 class="text-2xl font-bold tracking-tight text-white flex items-center gap-3">
          Edge Manufacturing HMI
          <span class="text-xs px-2 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 font-mono">
            RUST + VUE 3
          </span>
        </h1>
        <p class="text-xs text-slate-400 mt-1">Sub-millisecond IoT telemetry & computer vision pipeline</p>
      </div>

      <!-- WebSocket connection status -->
      <div class="flex items-center gap-2 text-xs font-mono px-3 py-1.5 rounded-lg border bg-slate-900" 
           :class="isConnected ? 'border-emerald-500/30 text-emerald-400' : 'border-red-500/30 text-red-400'">
        <span class="w-2 h-2 rounded-full" :class="isConnected ? 'bg-emerald-400 animate-ping' : 'bg-red-400'"></span>
        {{ isConnected ? 'EDGE STREAM CONNECTED' : 'DISCONNECTED' }}
      </div>
    </header>

    <!-- Main Dashboard Grid -->
    <main class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 space-y-6">
        <!-- Sensor Telemetry Gauges -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <SensorGauge title="Spindle Vibration" :value="vibration" unit="Hz" :warningThreshold="130" :criticalThreshold="150" />
          <SensorGauge title="Bearing Temperature" :value="temperature" unit="°C" :warningThreshold="75" :criticalThreshold="85" />
        </div>

        <!-- Vision Inspection Action Panel -->
        <div class="rounded-xl border border-slate-800 bg-slate-900/60 p-6 backdrop-blur-md">
          <div class="flex items-center justify-between mb-4">
            <div>
              <h2 class="text-sm font-semibold uppercase tracking-wider text-slate-300">Edge Vision Trigger</h2>
              <p class="text-xs text-slate-500">Run zero-copy ONNX frame inference on simulated camera buffer</p>
            </div>
            <button 
              @click="triggerInspection"
              class="px-4 py-2 bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-semibold text-xs rounded-lg transition-colors cursor-pointer"
            >
              Simulate Camera Frame Inspection
            </button>
          </div>
        </div>
      </div>

      <!-- Live Defect Alerts Feed -->
      <div class="lg:col-span-1">
        <DefectFeed :defects="defects" />
      </div>
    </main>
  </div>
</template>