<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { navigateToFolder, navigateBack, type ButtonConfig } from "./store";

  const { button } = $props<{ button: ButtonConfig }>();

  let isPressing = $state(false);

  async function handleTap() {
    isPressing = true;
    setTimeout(() => { isPressing = false; }, 150);

    try {
      if (button.type === "folder") {
        navigateToFolder(button.payload);
      } else if (button.type === "back") {
        navigateBack();
      } else {
        // Execute Tauri Rust IPC action
        await invoke("execute_action", {
          actionType: button.type,
          payload: button.payload,
        });
      }
    } catch (err) {
      console.error("IPC Action Execution Failed", err);
    }
  }
</script>

<button
  class="deck-key"
  class:pressing={isPressing}
  style="
    background: {button.bgColor || 'rgba(255, 255, 255, 0.03)'};
    --glow-color: {button.glowColor || 'rgba(255, 255, 255, 0.1)'};
  "
  onclick={handleTap}
>
  <div class="key-inner">
    <span class="key-icon">{button.icon || "⚙️"}</span>
    <span class="key-label">{button.label}</span>
  </div>
  <div class="glow-edge" style="background: {button.glowColor || 'transparent'}"></div>
</button>

<style>
  .deck-key {
    aspect-ratio: 1.1 / 1;
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.06);
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    transition: all 0.15s cubic-bezier(0.25, 0.8, 0.25, 1);
    padding: 12px;
    position: relative;
    overflow: hidden;
    width: 100%;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  /* Frosted Glass Highlight overlay */
  .deck-key::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 50%;
    background: linear-gradient(
      180deg,
      rgba(255, 255, 255, 0.08) 0%,
      transparent 100%
    );
    pointer-events: none;
  }

  /* Tactile transitions and scales */
  .deck-key:hover {
    border-color: rgba(255, 255, 255, 0.15);
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.25),
      0 0 12px var(--glow-color);
  }

  .deck-key:active,
  .deck-key.pressing {
    transform: scale(0.94) translateY(0);
    box-shadow: inset 0 2px 8px rgba(0, 0, 0, 0.4),
      0 0 4px var(--glow-color);
  }

  .key-inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    z-index: 2;
  }

  .key-icon {
    font-size: 2.2rem;
    margin-bottom: 6px;
    filter: drop-shadow(0 3px 6px rgba(0, 0, 0, 0.4));
    transition: transform 0.15s ease;
  }

  .deck-key:active .key-icon,
  .deck-key.pressing .key-icon {
    transform: scale(0.88);
  }

  .key-label {
    font-size: 0.8rem;
    font-weight: 600;
    text-align: center;
    color: var(--text-primary);
    letter-spacing: 0.02em;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
    max-width: 90px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Subtle bottom glow strip */
  .glow-edge {
    position: absolute;
    bottom: 0;
    left: 10%;
    width: 80%;
    height: 3px;
    border-top-left-radius: 4px;
    border-top-right-radius: 4px;
    opacity: 0.6;
    filter: blur(1px);
    z-index: 1;
    transition: height 0.2s ease, opacity 0.2s ease;
  }

  .deck-key:hover .glow-edge {
    height: 4px;
    opacity: 1;
    filter: blur(0.5px);
  }
</style>
