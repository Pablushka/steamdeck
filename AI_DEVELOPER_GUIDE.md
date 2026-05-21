# AI Developer Guide - Asus ScreenPad Plus Streamdeck Clone

This guide serves as a detailed step-by-step phased instruction manual for any AI agent or human developer to build the Streamdeck-like application on the `3840 x 1100` Asus Zenbook ScreenPad touch canvas.

---

## Technical Stack & Architecture
- **Frontend Framework**: Svelte (v3.x) + Vite + TypeScript.
- **Backend Native Host**: Tauri (v1.x) written in Rust.
- **Target Resolution**: `3840 x 1100` (physically ultra-wide 32:9).
- **DPI Scaling Strategy**: CSS percentage grids combined with dynamic viewport scaling.
- **Design Tokens**: Premium glassmorphic dark mode utilizing deep onyx backgrounds, frosted-glass panels, glowing thin borders, and vibrant active states.

---

## Phase 1: State Management & JSON Configuration Schema

### 1. Configuration File (`src/lib/deck_config.json`)
The entire action layout is driven by a single JSON configuration. Use this exact schema to support profiles, pages, custom actions, and folders (nested grids):

```json
{
  "activeProfile": "Default",
  "profiles": {
    "Default": {
      "name": "Default",
      "pages": [
        {
          "pageIndex": 0,
          "buttons": [
            {
              "id": "btn_launch_browser",
              "label": "Browser",
              "icon": "globe",
              "type": "launch",
              "payload": "https://google.com",
              "bgColor": "rgba(59, 130, 246, 0.2)",
              "glowColor": "#3b82f6"
            },
            {
              "id": "btn_mute_mic",
              "label": "Mute Mic",
              "icon": "microphone-slash",
              "type": "hotkey",
              "payload": "ctrl+shift+m",
              "bgColor": "rgba(239, 68, 68, 0.2)",
              "glowColor": "#ef4444"
            },
            {
              "id": "btn_folder_dev",
              "label": "Dev Tools",
              "icon": "folder",
              "type": "folder",
              "payload": "DevFolder",
              "bgColor": "rgba(168, 85, 247, 0.2)",
              "glowColor": "#a855f7"
            }
          ]
        }
      ]
    },
    "DevFolder": {
      "name": "Dev Tools Folder",
      "isFolder": true,
      "parentProfile": "Default",
      "pages": [
        {
          "pageIndex": 0,
          "buttons": [
            {
              "id": "btn_back",
              "label": "Back",
              "icon": "arrow-left",
              "type": "back",
              "payload": "",
              "bgColor": "rgba(107, 114, 128, 0.2)",
              "glowColor": "#6b7280"
            },
            {
              "id": "btn_launch_terminal",
              "label": "Terminal",
              "icon": "terminal",
              "type": "launch",
              "payload": "kitty",
              "bgColor": "rgba(34, 197, 94, 0.2)",
              "glowColor": "#22c55e"
            }
          ]
        }
      ]
    }
  }
}
```

### 2. Svelte State Store (`src/lib/store.ts`)
Create a Svelte store to manage dynamic operations:
```typescript
import { writable } from "svelte/store";
import defaultSchema from "./deck_config.json";

export interface ButtonConfig {
  id: string;
  label: string;
  icon: string;
  type: "launch" | "hotkey" | "folder" | "back" | "media" | "sys_control";
  payload: string;
  bgColor?: string;
  glowColor?: string;
}

export interface DeckPage {
  pageIndex: number;
  buttons: ButtonConfig[];
}

export interface ProfileConfig {
  name: string;
  isFolder?: boolean;
  parentProfile?: string;
  pages: DeckPage[];
}

// Global configurations and navigation
export const deckConfig = writable(defaultSchema);
export const currentProfileName = writable("Default");
export const currentPageIndex = writable(0);
export const systemStats = writable({ cpu: 0, ram: 0, gpu: 0, netDown: 0, netUp: 0 });
export const mediaState = writable({ volume: 50, brightness: 70, playing: false, track: "No media playing" });
```

---

## Phase 2: Touch Interface & UI Shell Layout

Restructure the application to fit the `3840 x 1100` aspect ratio. The screen is split horizontally into three zones:

### 1. Core Shell Grid (`src/App.svelte`)
```html
<script lang="ts">
  import SystemMonitor from "./lib/SystemMonitor.svelte";
  import ButtonGrid from "./lib/ButtonGrid.svelte";
  import ControlHub from "./lib/ControlHub.svelte";
  import { onMount } from "svelte";
  import { systemStats, mediaState } from "./lib/store";
  import { invoke } from "@tauri-apps/api/tauri";

  onMount(() => {
    // Poll system statistics from Rust every 2 seconds
    const interval = setInterval(async () => {
      try {
        const stats = await invoke("get_system_stats");
        systemStats.set(stats as any);
      } catch (err) {
        console.error("Failed to fetch system stats", err);
      }
    }, 2000);

    return () => clearInterval(interval);
  });
</script>

<div class="screenpad-app">
  <!-- Left Side: Hardware Analytics (20% width) -->
  <aside class="panel stats-panel">
    <SystemMonitor />
  </aside>

  <!-- Center: Action Key Deck (60% width) -->
  <main class="panel deck-panel">
    <ButtonGrid />
  </main>

  <!-- Right Side: Control Hub & Media Sliders (20% width) -->
  <aside class="panel controls-panel">
    <ControlHub />
  </aside>
</div>

<style>
  .screenpad-app {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: linear-gradient(135deg, #050608 0%, #0d0f15 100%);
    box-sizing: border-box;
    padding: 12px;
    gap: 16px;
  }

  .panel {
    border-radius: 18px;
    background: rgba(20, 24, 33, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    overflow: hidden;
  }

  .stats-panel {
    width: 20%;
    display: flex;
    flex-direction: column;
  }

  .deck-panel {
    width: 60%;
    display: flex;
    flex-direction: column;
  }

  .controls-panel {
    width: 20%;
    display: flex;
    flex-direction: column;
  }
</style>
```

### 2. Central Action Matrix (`src/lib/ButtonGrid.svelte`)
- Structure a grid of buttons. For ScreenPad (`3840 x 1100`), a **7 columns x 3 rows** or **8 columns x 3 rows** grid fits perfectly.
- Implement folder navigation by pushing the parent name onto a history stack, changing `currentProfileName`, and resetting `currentPageIndex = 0`.
- Support multiple pages with elegant indicators and left/right swiping logic.

---

## Phase 3: Rust Command Integration (`src-tauri`)

To run system actions, implement native system binds in Rust:

### 1. Crate Dependencies (`src-tauri/Cargo.toml`)
Ensure these dependencies are loaded:
```toml
[dependencies]
tauri = { version = "1.2.0", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sysinfo = "0.29" # For CPU, RAM metrics
# Add rdev or enigo if simulating precise keyboard shortcuts natively
```

### 2. Main Rust Engine (`src-tauri/src/main.rs`)
Implement the IPC handlers for resource monitoring and action triggering:

```rust
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use sysinfo::{CpuExt, System, SystemExt};
use std::sync::{Arc, Mutex};
use std::process::Command;

#[derive(Clone, Serialize)]
struct SystemStats {
    cpu: f32,
    ram: f32,
    gpu: f32,
    net_down: f64,
    net_up: f64,
}

// Keep a system handle wrapped in a state structure
struct SystemState {
    sys: System,
}

#[tauri::command]
fn get_system_stats(state: tauri::State<'_, Arc<Mutex<SystemState>>>) -> Result<SystemStats, String> {
    let mut state_guard = state.lock().map_err(|_| "Failed to lock mutex")?;
    state_guard.sys.refresh_all();
    
    // CPU Calculations
    let cpu_usage = state_guard.sys.global_cpu_info().cpu_usage();
    
    // RAM Calculations
    let total_mem = state_guard.sys.total_memory() as f32;
    let used_mem = state_guard.sys.used_memory() as f32;
    let ram_pct = (used_mem / total_mem) * 100.0;
    
    Ok(SystemStats {
        cpu: cpu_usage,
        ram: ram_pct,
        gpu: 0.0, // Platform specific GPU hook
        net_down: 0.0,
        net_up: 0.0,
    })
}

#[tauri::command]
fn execute_action(action_type: String, payload: String) -> Result<(), String> {
    match action_type.as_str() {
        "launch" => {
            // Launch web URLs or background terminal commands
            if payload.starts_with("http") {
                open::that(&payload).map_err(|e| e.to_string())?;
            } else {
                #[cfg(target_os = "windows")]
                Command::new("cmd").args(["/C", &payload]).spawn().map_err(|e| e.to_string())?;
                
                #[cfg(not(target_os = "windows"))]
                Command::new("sh").args(["-c", &payload]).spawn().map_err(|e| e.to_string())?;
            }
        }
        "hotkey" => {
            // Simulate hotkey pressing.
            // On Linux, you can issue shell command calls to xdotool:
            // e.g., Command::new("xdotool").args(["key", &payload]).output();
            // On Windows, use Powershell scripts or native rust crates (like enigo)
            println!("Simulating hotkey: {}", payload);
        }
        _ => return Err("Unknown action type".into()),
    }
    Ok(())
}

fn main() {
    let sys_state = Arc::new(Mutex::new(SystemState {
        sys: System::new_all(),
    }));

    tauri::Builder::default()
        .manage(sys_state)
        .invoke_handler(tauri::generate_handler![get_system_stats, execute_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Phase 4: Styling Tokens (`src/style.css`)
Ensure the visual layout conforms to high-fidelity glassmorphism:
- Set viewport boundaries to exactly cover a single-screen region with `overflow: hidden`.
- Use custom styling variables to control glows, rounding (`border-radius`), and font metrics.
- Utilize grid setups where individual cards hold touch responses with subtle scaling properties.

---

## Developer Commands & Execution Checklist
1. **Initialize Project Libraries**:
   - `npm install` inside the project root directory.
2. **Launch Development Sandbox**:
   - Start the Svelte server and the Tauri engine simultaneously:
     `npm run tauri dev`
3. **Compile Release Binary**:
   - To build the production app optimized for the Asus ScreenPad Plus:
     `npm run tauri build`
