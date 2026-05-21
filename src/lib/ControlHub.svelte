<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { mediaState } from "./store";

  async function adjustVolume(direction: "up" | "down" | "mute") {
    try {
      await invoke("adjust_system_setting", {
        settingType: "volume",
        direction,
      });
      
      // Update local store state for instant visual feedback
      mediaState.update(s => {
        let newVol = s.volume;
        if (direction === "up") newVol = Math.min(100, s.volume + 5);
        if (direction === "down") newVol = Math.max(0, s.volume - 5);
        return { ...s, volume: newVol };
      });
    } catch (err) {
      console.error("Failed to adjust volume", err);
    }
  }

  async function adjustBrightness(direction: "up" | "down") {
    try {
      await invoke("adjust_system_setting", {
        settingType: "brightness",
        direction,
      });

      mediaState.update(s => {
        let newBright = s.brightness;
        if (direction === "up") newBright = Math.min(100, s.brightness + 10);
        if (direction === "down") newBright = Math.max(0, s.brightness - 10);
        return { ...s, brightness: newBright };
      });
    } catch (err) {
      console.error("Failed to adjust brightness", err);
    }
  }

  async function triggerPlayer(action: "play-pause" | "next" | "previous") {
    try {
      // Toggle state locally for immediate tactile feedback
      if (action === "play-pause") {
        mediaState.update(s => ({ ...s, playing: !s.playing }));
      }
      
      // Execute global Linux media controls using playerctl shell executor
      const cmd = `playerctl ${action}`;
      await invoke("execute_action", {
        actionType: "launch",
        payload: cmd,
      });
    } catch (err) {
      console.error("Failed to trigger playerctl", err);
    }
  }
</script>

<div class="control-hub">
  <h2 class="stats-title">CONTROLS</h2>

  <!-- Sliders panel -->
  <div class="sliders-section">
    <!-- Volume Slider Section -->
    <div class="slider-card">
      <div class="slider-header">
        <span class="card-icon">🔊</span>
        <span class="card-title">VOLUME</span>
        <span class="card-val">{$mediaState.volume}%</span>
      </div>
      <div class="slider-controls">
        <button class="adjust-btn" onclick={() => adjustVolume("down")}>−</button>
        <div class="slider-bar-track">
          <div class="slider-bar-fill" style="width: {$mediaState.volume}%"></div>
        </div>
        <button class="adjust-btn" onclick={() => adjustVolume("up")}>+</button>
      </div>
    </div>

    <!-- Brightness Slider Section -->
    <div class="slider-card">
      <div class="slider-header">
        <span class="card-icon">🔆</span>
        <span class="card-title">BRIGHTNESS</span>
        <span class="card-val">{$mediaState.brightness}%</span>
      </div>
      <div class="slider-controls">
        <button class="adjust-btn" onclick={() => adjustBrightness("down")}>−</button>
        <div class="slider-bar-track">
          <div class="slider-bar-fill bright-fill" style="width: {$mediaState.brightness}%"></div>
        </div>
        <button class="adjust-btn" onclick={() => adjustBrightness("up")}>+</button>
      </div>
    </div>
  </div>

  <!-- Media Player Panel -->
  <div class="player-panel">
    <div class="track-info">
      <span class="music-icon">🎵</span>
      <div class="track-text">
        <span class="track-name">{$mediaState.track}</span>
        <span class="track-artist">System Media Output</span>
      </div>
    </div>
    <div class="player-buttons">
      <button class="media-btn" onclick={() => triggerPlayer("previous")}>⏮️</button>
      <button class="media-btn play-pause-btn" onclick={() => triggerPlayer("play-pause")}>
        {$mediaState.playing ? "⏸️" : "▶️"}
      </button>
      <button class="media-btn" onclick={() => triggerPlayer("next")}>⏭️</button>
    </div>
  </div>
</div>

<style>
  .control-hub {
    display: flex;
    flex-direction: column;
    height: 100%;
    justify-content: space-between;
  }

  .sliders-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
    flex-grow: 1;
    justify-content: center;
  }

  .slider-card {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.04);
    border-radius: 12px;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .slider-header {
    display: flex;
    align-items: center;
    font-size: 0.72rem;
    font-weight: 700;
    color: var(--text-secondary);
    letter-spacing: 0.05em;
  }

  .card-icon {
    margin-right: 6px;
    font-size: 0.9rem;
  }

  .card-title {
    flex-grow: 1;
  }

  .card-val {
    color: var(--text-primary);
  }

  .slider-controls {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .adjust-btn {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    color: var(--text-primary);
    width: 24px;
    height: 24px;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: none;
  }

  .adjust-btn:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .adjust-btn:active {
    transform: scale(0.9);
  }

  .slider-bar-track {
    flex-grow: 1;
    height: 6px;
    background: rgba(255, 255, 255, 0.06);
    border-radius: 3px;
    overflow: hidden;
    position: relative;
  }

  .slider-bar-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6 0%, #06b6d4 100%);
    border-radius: 3px;
    transition: width 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 0 8px rgba(59, 130, 246, 0.5);
  }

  .slider-bar-fill.bright-fill {
    background: linear-gradient(90deg, #eab308 0%, #f97316 100%);
    box-shadow: 0 0 8px rgba(234, 179, 8, 0.5);
  }

  /* Media Player Panel */
  .player-panel {
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    padding-top: 12px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .track-info {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(255, 255, 255, 0.01);
    padding: 6px 10px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.02);
  }

  .music-icon {
    font-size: 1.1rem;
    animation: pulse 2s infinite ease-in-out;
  }

  .track-text {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .track-name {
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .track-artist {
    font-size: 0.6rem;
    color: var(--text-muted);
  }

  .player-buttons {
    display: flex;
    justify-content: center;
    gap: 12px;
  }

  .media-btn {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.05);
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.95rem;
    cursor: pointer;
    box-shadow: none;
    transition: all 0.2s ease;
  }

  .media-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    transform: translateY(-1px);
    box-shadow: 0 0 6px rgba(255, 255, 255, 0.1);
  }

  .media-btn:active {
    transform: scale(0.9) translateY(0);
  }

  .play-pause-btn {
    background: rgba(168, 85, 247, 0.15);
    border-color: rgba(168, 85, 247, 0.3);
    width: 36px;
    height: 36px;
    font-size: 1rem;
  }

  .play-pause-btn:hover {
    background: rgba(168, 85, 247, 0.25);
    box-shadow: 0 0 8px rgba(168, 85, 247, 0.4);
  }

  @keyframes pulse {
    0%, 100% { transform: scale(1); opacity: 0.8; }
    50% { transform: scale(1.1); opacity: 1; filter: drop-shadow(0 0 4px #ec4899); }
  }
</style>
