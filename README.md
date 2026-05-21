# Steamdeck ScreenPad Control Surface

Desktop control surface built for an ultra-wide 3840x1100 touch area using Svelte 5 + Tauri 2.

## Current Stack

- Frontend: Svelte 5 (runes mode), Vite 8, TypeScript 6
- Backend: Tauri 2, Rust
- Package manager: pnpm
- UI model: JSON-driven button deck with folders/pages
- Layout model: fixed 3840x1100 design canvas with dynamic viewport scaling

## Project Structure

- `src/App.svelte`: app shell, DPI-aware scaling wrapper, telemetry polling
- `src/lib/ButtonGrid.svelte`: active profile/page deck rendering
- `src/lib/ControlHub.svelte`: media and system controls
- `src/lib/SystemMonitor.svelte`: CPU/RAM visual telemetry
- `src/lib/store.ts`: stores and folder navigation state
- `src/lib/deck_config.json`: profile/page/button config source
- `src-tauri/src/lib.rs`: Tauri commands and OS actions

## Runtime Features

- Polls system telemetry from Rust via `get_system_stats`
- Executes deck actions via `execute_action`
- Adjusts volume/brightness via `adjust_system_setting`
- Supports nested folder navigation in the deck
- Auto-scales UI to current viewport while preserving 3840x1100 proportions

## Development

Install dependencies:

```bash
pnpm install
```

Run frontend dev server:

```bash
pnpm run dev
```

Run Tauri app:

```bash
pnpm run tauri dev
```

Build frontend:

```bash
pnpm run build
```

Run type and Svelte checks:

```bash
pnpm run check
```

Build desktop bundle:

```bash
pnpm run tauri build
```

## Notes

- Svelte compiler runes mode is enabled in `svelte.config.js`.
- The Tauri backend currently uses `tauri-plugin-opener` and `tauri-plugin-shell`.
- Linux control commands use tools like `pactl`, `brightnessctl`, `xbacklight`, and `xdotool` when available.
