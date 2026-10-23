<script setup lang="ts">
// ============================================================================
// EXECUTION PATH: Vision Inspection Alert Feed
// Displays real-time defect detections emitted by the Rust ONNX engine.
// ============================================================================

export interface Defect {
  id: string
  defectType: string
  confidence: number
  timestamp: string
}

defineProps<{
  defects: Defect[]
}>()
</script>

<template>
  <div class="rounded-xl border border-slate-800 bg-slate-900/60 p-5 backdrop-blur-md flex flex-col h-full">
    <h3 class="text-sm font-semibold uppercase tracking-wider text-slate-400 mb-4 flex items-center gap-2">
      <span class="w-2 h-2 rounded-full bg-cyan-400"></span>
      Computer Vision Defect Alerts
    </h3>

    <!-- SAD PATH: Render empty state if no defects detected -->
    <div v-if="defects.length === 0" class="flex-1 flex flex-col items-center justify-center py-12 text-center text-slate-500">
      <svg class="w-10 h-10 mb-2 stroke-current opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <p class="text-xs">No active surface defects detected on conveyor line</p>
    </div>

    <!-- HOT PATH: Scrollable feed of live ONNX inspection alerts -->
    <div v-else class="space-y-3 overflow-y-auto max-h-[380px] pr-1">
      <div 
        v-for="item in defects" 
        :key="item.id"
        class="flex items-center justify-between p-3.5 rounded-lg bg-slate-800/40 border border-slate-700/50 hover:border-slate-600 transition-colors"
      >
        <div>
          <p class="text-sm font-medium text-slate-200">{{ item.defectType }}</p>
          <span class="text-xs font-mono text-slate-500">{{ item.timestamp }}</span>
        </div>
        <div class="text-right">
          <span class="inline-block px-2 py-1 rounded bg-red-500/10 text-red-400 font-mono text-xs font-bold border border-red-500/20">
            {{ (item.confidence * 100).toFixed(0) }}% Confidence
          </span>
        </div>
      </div>
    </div>
  </div>
</template>