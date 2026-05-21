
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let name = $state("");
  let greetMsg = $state("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." value={name} oninput={e => name = e.target.value} />
    <button onclick={greet}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>