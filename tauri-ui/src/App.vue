<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Types
interface StatusResponse {
  enabled: boolean
  has_permission: boolean
  has_optimized: boolean
}

interface OperationResult {
  success: boolean
  message: string
}

interface SpeedTestResult {
  domain: string
  ip: string
  latency_ms: number
  quality: string
  color: [number, number, number]
}

interface Particle {
  x: number
  y: number
  vx: number
  vy: number
  size: number
  alpha: number
  color: string
}

// State
const status = ref<StatusResponse>({ enabled: false, has_permission: false, has_optimized: false })
const speedTestResults = ref<SpeedTestResult[]>([])
const isTesting = ref(false)
const message = ref('')
const messageType = ref<'success' | 'error' | 'info'>('info')

// Particle system
const particles = ref<Particle[]>([])
const canvasRef = ref<HTMLCanvasElement | null>(null)
let animationId: number | null = null

function initParticles() {
  const colors = ['#3b82f6', '#10b981', '#f59e0b', '#8b5cf6', '#ec4899']
  for (let i = 0; i < 50; i++) {
    particles.value.push({
      x: Math.random() * 520,
      y: Math.random() * 720,
      vx: (Math.random() - 0.5) * 0.5,
      vy: (Math.random() - 0.5) * 0.5,
      size: Math.random() * 3 + 1,
      alpha: Math.random() * 0.5 + 0.1,
      color: colors[Math.floor(Math.random() * colors.length)]
    })
  }
}

function animateParticles() {
  const canvas = canvasRef.value
  if (!canvas) return
  
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  
  particles.value.forEach(p => {
    p.x += p.vx
    p.y += p.vy
    
    if (p.x < 0) p.x = canvas.width
    if (p.x > canvas.width) p.x = 0
    if (p.y < 0) p.y = canvas.height
    if (p.y > canvas.height) p.y = 0
    
    ctx.beginPath()
    ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2)
    ctx.fillStyle = p.color
    ctx.globalAlpha = p.alpha
    ctx.fill()
  })
  
  // Draw connections
  ctx.globalAlpha = 0.1
  ctx.strokeStyle = '#3b82f6'
  particles.value.forEach((p1, i) => {
    particles.value.slice(i + 1).forEach(p2 => {
      const dx = p1.x - p2.x
      const dy = p1.y - p2.y
      const dist = Math.sqrt(dx * dx + dy * dy)
      if (dist < 100) {
        ctx.beginPath()
        ctx.moveTo(p1.x, p1.y)
        ctx.lineTo(p2.x, p2.y)
        ctx.stroke()
      }
    })
  })
  
  ctx.globalAlpha = 1
  animationId = requestAnimationFrame(animateParticles)
}

// Computed
const statusText = computed(() => {
  if (!status.value.has_permission) return '无权限'
  if (status.value.enabled) {
    return status.value.has_optimized ? '已启用 (已优化)' : '已启用'
  }
  return '未启用'
})

const statusClass = computed(() => {
  if (!status.value.has_permission) return 'status-warning'
  return status.value.enabled ? 'status-enabled' : 'status-disabled'
})

// Methods
async function refreshStatus() {
  try {
    status.value = await invoke<StatusResponse>('get_status')
  } catch (e) {
    console.error('Failed to get status:', e)
  }
}

async function runSpeedTest() {
  isTesting.value = true
  message.value = ''
  try {
    speedTestResults.value = await invoke<SpeedTestResult[]>('run_speed_test')
    await refreshStatus()
    showMessage('测速完成！', 'success')
  } catch (e) {
    showMessage(`测速失败: ${e}`, 'error')
  } finally {
    isTesting.value = false
  }
}

async function enableAcceleration(optimized: boolean) {
  try {
    const result = await invoke<OperationResult>(
      optimized ? 'enable_optimized' : 'enable_acceleration'
    )
    if (result.success) {
      showMessage('加速已启用！', 'success')
      await refreshStatus()
    } else {
      showMessage(result.message, 'error')
    }
  } catch (e) {
    showMessage(`启用失败: ${e}`, 'error')
  }
}

async function disableAcceleration() {
  try {
    const result = await invoke<OperationResult>('disable_acceleration')
    if (result.success) {
      showMessage('加速已禁用', 'success')
      await refreshStatus()
    } else {
      showMessage(result.message, 'error')
    }
  } catch (e) {
    showMessage(`禁用失败: ${e}`, 'error')
  }
}

async function flushDns() {
  try {
    const result = await invoke<OperationResult>('flush_dns')
    showMessage(result.success ? 'DNS缓存已刷新！' : result.message, result.success ? 'success' : 'error')
  } catch (e) {
    showMessage(`刷新DNS失败: ${e}`, 'error')
  }
}

async function openHostsFolder() {
  await invoke('open_hosts_folder')
}

async function openGitHub() {
  await invoke('open_github')
}

function showMessage(msg: string, type: 'success' | 'error' | 'info') {
  message.value = msg
  messageType.value = type
  setTimeout(() => { message.value = '' }, 3000)
}

function getResultColor(color: [number, number, number]) {
  return `rgb(${color[0]}, ${color[1]}, ${color[2]})`
}

function getQualityText(quality: string) {
  const map: Record<string, string> = {
    'Excellent': '极佳',
    'Good': '良好',
    'Fair': '一般',
    'Slow': '较慢',
    'Very Slow': '很慢'
  }
  return map[quality] || quality
}

onMounted(() => {
  refreshStatus()
  initParticles()
  animateParticles()
})

onUnmounted(() => {
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
})
</script>

<template>
  <div class="app">
    <!-- Particle Canvas -->
    <canvas ref="canvasRef" class="particles" width="520" height="720"></canvas>
    
    <!-- Header -->
    <header class="header">
      <div class="logo">Free to GitHub</div>
      <div class="subtitle">GitHub 访问加速工具</div>
    </header>

    <!-- Permission Warning -->
    <div v-if="!status.has_permission" class="warning-banner">
      <span class="warning-icon">!</span>
      <div>
        <strong>需要管理员权限</strong>
        <p>请以管理员身份运行程序</p>
      </div>
    </div>

    <!-- Status Card -->
    <div class="status-card" :class="statusClass">
      <div class="status-icon">
        <span v-if="status.enabled">开</span>
        <span v-else>关</span>
      </div>
      <div class="status-text">{{ statusText }}</div>
    </div>

    <!-- Speed Test Results -->
    <div v-if="speedTestResults.length > 0" class="results-card">
      <div class="results-header">测速结果</div>
      <div class="results-list">
        <div v-for="result in speedTestResults.slice(0, 6)" :key="result.domain" class="result-item">
          <span class="domain">{{ result.domain }}</span>
          <span class="latency" :style="{ color: getResultColor(result.color) }">
            {{ result.latency_ms }}ms - {{ getQualityText(result.quality) }}
          </span>
        </div>
      </div>
    </div>

    <!-- Testing Indicator -->
    <div v-if="isTesting" class="testing-indicator">
      <div class="spinner"></div>
      <span>正在测速中...</span>
    </div>

    <!-- Main Actions -->
    <div class="actions">
      <button class="btn btn-speed" :disabled="isTesting" @click="runSpeedTest">
        测速
      </button>
      <button 
        class="btn btn-enable" 
        :disabled="!status.has_permission || isTesting"
        @click="enableAcceleration(status.has_optimized)"
      >
        {{ status.has_optimized ? '智能启用' : '启用加速' }}
      </button>
      <button 
        class="btn btn-disable" 
        :disabled="!status.has_permission || isTesting"
        @click="disableAcceleration"
      >
        禁用
      </button>
    </div>

    <!-- Secondary Actions -->
    <div class="secondary-actions">
      <button class="btn-secondary" @click="flushDns">刷新DNS</button>
      <button class="btn-secondary" @click="openHostsFolder">Hosts文件</button>
      <button 
        class="btn-secondary btn-github" 
        :disabled="!status.enabled"
        @click="openGitHub"
      >
        打开GitHub
      </button>
    </div>

    <!-- Message Toast -->
    <transition name="fade">
      <div v-if="message" class="toast" :class="messageType">
        {{ message }}
      </div>
    </transition>

    <!-- Footer -->
    <footer class="footer">
      <span v-if="status.has_optimized">正在使用优化后的IP</span>
      <span v-else>建议先测速再启用加速</span>
    </footer>
  </div>
</template>

<style>
:root {
  --bg-primary: #f8fafc;
  --bg-secondary: #ffffff;
  --bg-card: #ffffff;
  --text-primary: #1e293b;
  --text-secondary: #64748b;
  --accent-green: #10b981;
  --accent-red: #ef4444;
  --accent-orange: #f59e0b;
  --accent-blue: #3b82f6;
  --border-color: #e2e8f0;
  --shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Microsoft YaHei', sans-serif;
  background: linear-gradient(135deg, #e0e7ff 0%, #fce7f3 50%, #dbeafe 100%);
  color: var(--text-primary);
  overflow: hidden;
}

.app {
  min-height: 100vh;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  position: relative;
}

.particles {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 0;
}

.header, .warning-banner, .status-card, .results-card, .actions, .secondary-actions, .testing-indicator, .footer {
  position: relative;
  z-index: 1;
}

.header {
  text-align: center;
  padding: 20px 0;
}

.logo {
  font-size: 32px;
  font-weight: 800;
  background: linear-gradient(135deg, var(--accent-blue), #8b5cf6, var(--accent-green));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  text-shadow: 0 2px 10px rgba(59, 130, 246, 0.3);
}

.subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin-top: 6px;
  font-weight: 500;
}

.warning-banner {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 18px;
  background: linear-gradient(135deg, #fef3c7, #fde68a);
  border: 1px solid var(--accent-orange);
  border-radius: 12px;
  box-shadow: var(--shadow);
}

.warning-icon {
  width: 28px;
  height: 28px;
  background: var(--accent-orange);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  font-size: 16px;
}

.warning-banner p {
  font-size: 12px;
  color: #92400e;
  margin-top: 2px;
}

.warning-banner strong {
  color: #78350f;
}

.status-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 32px;
  background: var(--bg-card);
  border-radius: 20px;
  border: 2px solid var(--border-color);
  box-shadow: var(--shadow);
  transition: all 0.3s ease;
}

.status-card.status-enabled {
  border-color: var(--accent-green);
  background: linear-gradient(135deg, #ecfdf5, #d1fae5);
  box-shadow: 0 0 30px rgba(16, 185, 129, 0.2);
}

.status-card.status-disabled {
  border-color: var(--border-color);
}

.status-card.status-warning {
  border-color: var(--accent-orange);
  background: linear-gradient(135deg, #fffbeb, #fef3c7);
}

.status-icon {
  width: 90px;
  height: 90px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  font-weight: 800;
  margin-bottom: 14px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.15);
}

.status-enabled .status-icon {
  background: linear-gradient(135deg, var(--accent-green), #059669);
  color: white;
}

.status-disabled .status-icon {
  background: linear-gradient(135deg, #cbd5e1, #94a3b8);
  color: white;
}

.status-warning .status-icon {
  background: linear-gradient(135deg, var(--accent-orange), #d97706);
  color: white;
}

.status-text {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
}

.results-card {
  background: var(--bg-card);
  border-radius: 16px;
  border: 1px solid var(--border-color);
  padding: 18px;
  box-shadow: var(--shadow);
}

.results-header {
  font-size: 15px;
  color: var(--text-secondary);
  margin-bottom: 14px;
  font-weight: 600;
}

.results-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  max-height: 150px;
  overflow-y: auto;
  padding-right: 8px;
}

.results-list::-webkit-scrollbar {
  width: 6px;
}

.results-list::-webkit-scrollbar-track {
  background: #f1f5f9;
  border-radius: 3px;
}

.results-list::-webkit-scrollbar-thumb {
  background: var(--accent-blue);
  border-radius: 3px;
}

.results-list::-webkit-scrollbar-thumb:hover {
  background: #2563eb;
}

.result-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  padding: 6px 0;
  border-bottom: 1px solid #f1f5f9;
}

.result-item:last-child {
  border-bottom: none;
}

.domain {
  color: var(--text-secondary);
  max-width: 180px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.latency {
  font-weight: 600;
}

.testing-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 18px;
  color: var(--accent-blue);
  font-weight: 500;
}

.spinner {
  width: 22px;
  height: 22px;
  border: 3px solid var(--border-color);
  border-top-color: var(--accent-blue);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.actions {
  display: flex;
  gap: 12px;
  margin-top: 8px;
}

.btn {
  flex: 1;
  padding: 16px 20px;
  border: none;
  border-radius: 14px;
  font-size: 16px;
  font-weight: 700;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: var(--shadow);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
}

.btn-speed {
  background: linear-gradient(135deg, var(--accent-orange), #ea580c);
  color: white;
}

.btn-speed:hover:not(:disabled) {
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(245, 158, 11, 0.4);
}

.btn-enable {
  background: linear-gradient(135deg, var(--accent-green), #059669);
  color: white;
}

.btn-enable:hover:not(:disabled) {
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(16, 185, 129, 0.4);
}

.btn-disable {
  background: linear-gradient(135deg, var(--accent-red), #dc2626);
  color: white;
}

.btn-disable:hover:not(:disabled) {
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(239, 68, 68, 0.4);
}

.secondary-actions {
  display: flex;
  gap: 10px;
}

.btn-secondary {
  flex: 1;
  padding: 14px;
  background: var(--bg-secondary);
  border: 2px solid var(--border-color);
  border-radius: 12px;
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: var(--shadow);
}

.btn-secondary:hover:not(:disabled) {
  background: #f1f5f9;
  border-color: var(--accent-blue);
  color: var(--accent-blue);
  transform: translateY(-2px);
}

.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-github:not(:disabled) {
  border-color: var(--accent-blue);
  color: var(--accent-blue);
  background: #eff6ff;
}

.toast {
  position: fixed;
  bottom: 80px;
  left: 50%;
  transform: translateX(-50%);
  padding: 14px 28px;
  border-radius: 12px;
  font-size: 15px;
  font-weight: 600;
  z-index: 1000;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
}

.toast.success {
  background: linear-gradient(135deg, var(--accent-green), #059669);
  color: white;
}

.toast.error {
  background: linear-gradient(135deg, var(--accent-red), #dc2626);
  color: white;
}

.toast.info {
  background: linear-gradient(135deg, var(--accent-blue), #2563eb);
  color: white;
}

.fade-enter-active, .fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(20px);
}

.footer {
  margin-top: auto;
  text-align: center;
  font-size: 13px;
  color: var(--text-secondary);
  padding: 16px 0;
  font-weight: 500;
}
</style>