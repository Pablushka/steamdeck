<script lang="ts">
  import { systemStats } from "./store";
  
  // Circumference of our SVG dials: 2 * PI * r (r=36) => ~226.19
  const circumference = 2 * Math.PI * 36;
  
  const cpuOffset = $derived(circumference - ($systemStats.cpu / 100) * circumference);
  const ramOffset = $derived(circumference - ($systemStats.ram / 100) * circumference);
</script>

<div class="monitor-container">
  <h2 class="stats-title">TELEMETRY</h2>
  
  <div class="metrics-grid">
    <!-- CPU Radial Gauge -->
    <div class="gauge-card">
      <div class="svg-container">
        <svg class="radial-svg" viewBox="0 0 90 90">
          <defs>
            <linearGradient id="cpuGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#3b82f6" />
              <stop offset="100%" stop-color="#8b5cf6" />
            </linearGradient>
          </defs>
          <!-- Track Circle -->
          <circle class="gauge-track" cx="45" cy="45" r="36" />
          <!-- Active Circle -->
          <circle 
            class="gauge-active cpu-active" 
            cx="45" 
            cy="45" 
            r="36" 
            stroke="url(#cpuGrad)"
            stroke-dasharray={circumference}
            stroke-dashoffset={cpuOffset}
          />
        </svg>
        <div class="gauge-label">
          <span class="gauge-val">{Math.round($systemStats.cpu)}%</span>
          <span class="gauge-sub">CPU</span>
        </div>
      </div>
    </div>

    <!-- RAM Radial Gauge -->
    <div class="gauge-card">
      <div class="svg-container">
        <svg class="radial-svg" viewBox="0 0 90 90">
          <defs>
            <linearGradient id="ramGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#ec4899" />
              <stop offset="100%" stop-color="#f43f5e" />
            </linearGradient>
          </defs>
          <!-- Track Circle -->
          <circle class="gauge-track" cx="45" cy="45" r="36" />
          <!-- Active Circle -->
          <circle 
            class="gauge-active ram-active" 
            cx="45" 
            cy="45" 
            r="36" 
            stroke="url(#ramGrad)"
            stroke-dasharray={circumference}
            stroke-dashoffset={ramOffset}
          />
        </svg>
        <div class="gauge-label">
          <span class="gauge-val">{Math.round($systemStats.ram)}%</span>
          <span class="gauge-sub">RAM</span>
        </div>
      </div>
    </div>
  </div>

  <div class="hardware-logs">
    <div class="log-row">
      <span class="log-dot green"></span>
      <span class="log-text">Tauri Core Engine Status</span>
      <span class="log-val">Active</span>
    </div>
    <div class="log-row">
      <span class="log-dot blue"></span>
      <span class="log-text">ScreenPad Touch Canvas</span>
      <span class="log-val">3840x1100</span>
    </div>
  </div>
</div>

<style>
  .monitor-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    justify-content: space-between;
  }

  .metrics-grid {
    display: flex;
    flex-grow: 1;
    justify-content: space-around;
    align-items: center;
    gap: 12px;
  }

  .gauge-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: relative;
    width: 100px;
    height: 100px;
  }

  .svg-container {
    position: relative;
    width: 90px;
    height: 90px;
  }

  .radial-svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }

  circle {
    fill: transparent;
    stroke-width: 7;
  }

  .gauge-track {
    stroke: rgba(255, 255, 255, 0.04);
  }

  .gauge-active {
    stroke-linecap: round;
    transition: stroke-dashoffset 0.6s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .cpu-active {
    filter: drop-shadow(0 0 6px rgba(139, 92, 246, 0.5));
  }

  .ram-active {
    filter: drop-shadow(0 0 6px rgba(244, 63, 94, 0.5));
  }

  .gauge-label {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .gauge-val {
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
  }

  .gauge-sub {
    font-size: 0.65rem;
    font-weight: 600;
    color: var(--text-secondary);
    letter-spacing: 0.05em;
    margin-top: 2px;
  }

  .hardware-logs {
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    padding-top: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .log-row {
    display: flex;
    align-items: center;
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .log-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    margin-right: 8px;
  }

  .log-dot.green {
    background: #10b981;
    box-shadow: 0 0 6px #10b981;
  }

  .log-dot.blue {
    background: #3b82f6;
    box-shadow: 0 0 6px #3b82f6;
  }

  .log-text {
    flex-grow: 1;
  }

  .log-val {
    font-weight: 600;
    color: var(--text-primary);
  }
</style>
