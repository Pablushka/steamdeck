<script lang="ts">
  import { onMount } from "svelte";
  import { systemStats } from "./lib/store";
  import SystemMonitor from "./lib/SystemMonitor.svelte";
  import ButtonGrid from "./lib/ButtonGrid.svelte";
  import ControlHub from "./lib/ControlHub.svelte";
  import { invoke } from "@tauri-apps/api/core";

  const designWidth = 3840;
  const designHeight = 1100;

  let viewportScale = $state(1);
  let offsetX = $state(0);
  let offsetY = $state(0);

  function updateViewportScale() {
    const widthRatio = window.innerWidth / designWidth;
    const heightRatio = window.innerHeight / designHeight;

    viewportScale = Math.min(widthRatio, heightRatio);
    offsetX = (window.innerWidth - designWidth * viewportScale) / 2;
    offsetY = (window.innerHeight - designHeight * viewportScale) / 2;
  }

  onMount(() => {
    updateViewportScale();

    // Flag to track whether Tauri API is available (e.g. if run in browser preview)
    const isTauri = typeof window !== "undefined" && (window as any).__TAURI_METADATA__ !== undefined;

    const handleResize = () => updateViewportScale();
    window.addEventListener("resize", handleResize);

    // Periodic telemetry update from Rust
    const interval = setInterval(async () => {
      try {
        if (isTauri) {
          const stats = await invoke("get_system_stats");
          systemStats.set(stats as any);
        } else {
          // Dynamic mock hardware data for clean web-browser preview/debugging
          systemStats.update(s => ({
            cpu: Math.max(5, Math.min(95, s.cpu + (Math.random() - 0.5) * 15)),
            ram: Math.max(20, Math.min(90, s.ram + (Math.random() - 0.5) * 4)),
            gpu: 0.0,
            netDown: 0.0,
            netUp: 0.0
          }));
        }
      } catch (err) {
        console.error("Failed to query hardware telemetry", err);
      }
    }, 2000);

    return () => {
      clearInterval(interval);
      window.removeEventListener("resize", handleResize);
    };
  });
</script>

<div class="viewport-shell">
  <div
    class="viewport-stage"
    style={`--viewport-scale: ${viewportScale}; --offset-x: ${offsetX}px; --offset-y: ${offsetY}px;`}
  >
    <div class="screenpad-app">
      <!-- Left Side Panel: System resources dials -->
      <aside class="panel stats-panel">
        <SystemMonitor />
      </aside>

      <!-- Center Panel: The Streamdeck active button grid -->
      <main class="panel deck-panel">
        <ButtonGrid />
      </main>

      <!-- Right Side Panel: System Volume, Brightness, and Media controls -->
      <aside class="panel controls-panel">
        <ControlHub />
      </aside>
    </div>
  </div>
</div>

<style>
  .viewport-shell {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    position: relative;
  }

  .viewport-stage {
    position: absolute;
    transform-origin: top left;
    transform: translate3d(var(--offset-x), var(--offset-y), 0) scale(var(--viewport-scale));
    will-change: transform;
  }

  .screenpad-app {
    width: 3840px;
    height: 1100px;
  }
</style>
