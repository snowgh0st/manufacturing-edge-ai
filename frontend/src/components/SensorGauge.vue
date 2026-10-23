<script setup lang="ts">
// ============================================================================
// EXECUTION PATH: Reactive Telemetry Gauge
// Displays real-time sensor metrics (Vibration Hz & Temp °C) with dynamic 
// status color indicators based on operational safety thresholds.
// ============================================================================

defineProps<{
  title: string
  value: number
  unit: string
  warningThreshold: number
  criticalThreshold: number
}>()

// HOT PATH: Helper function to determine badge colors dynamically
const getStatusColor = (val: number, warn: number, crit: number) => {
  if (val >= crit) return 'bg-red-500/20 text-red-400 border-red-500/40'
  if (val >= warn) return 'bg-amber-500/20 text-amber-400 border-amber-500/40'
  return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/40'
}
</script>

<template>
  <div class="rounded-xl border border-slate-800 bg-slate-900/60 p-5 backdrop-blur-md">
    <div class="flex items-center justify-between mb-3">
      <span class="text-xs font-semibold uppercase tracking-wider text-slate-400">{{ title }}</span>
      <span 
        class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-medium border"
        :class="getStatusColor(value, warningThreshold, criticalThreshold)"
      >
        <span class="w-1.5 h-1.5 rounded-full bg-current animate-pulse"></span>
        {{ value >= criticalThreshold ? 'CRITICAL' : value >= warningThreshold ? 'WARNING' : 'NOMINAL' }}
      </span>
    </div>
    <div class="flex items-baseline gap-2">
      <span class="text-3xl font-bold font-mono text-slate-100">{{ value.toFixed(1) }}</span>
      <span class="text-sm text-slate-400 font-medium">{{ unit }}</span>
    </div>
  </div>
</template>