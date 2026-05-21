<script lang="ts">
  import ButtonKey from "./ButtonKey.svelte";
  import { deckConfig, currentProfileName, currentPageIndex, type ButtonConfig } from "./store";

  // Grid settings: 8 columns x 2 rows = 16 keys total per page
  const cols = 8;
  const rows = 2;
  const maxSlots = cols * rows;

  const activeProfile = $derived.by(() => $deckConfig.profiles[$currentProfileName] || $deckConfig.profiles["Default"]);
  const activePages = $derived.by(() => activeProfile.pages || []);
  const currentPage = $derived.by(() => activePages[$currentPageIndex] || activePages[0] || { buttons: [] });

  // Fill the grid array up to 16 slots with buttons or empty placeholders
  const gridItems = $derived.by(() => {
    const list: (ButtonConfig | null)[] = [...(currentPage.buttons || [])];
    while (list.length < maxSlots) {
      list.push(null);
    }
    return list.slice(0, maxSlots);
  });

  function nextPage() {
    if ($currentPageIndex < activePages.length - 1) {
      currentPageIndex.update(i => i + 1);
    }
  }

  function prevPage() {
    if ($currentPageIndex > 0) {
      currentPageIndex.update(i => i - 1);
    }
  }
</script>

<div class="deck-container">
  <!-- Deck Header Section -->
  <header class="deck-header">
    <div class="header-left">
      <span class="profile-badge">PROFILE</span>
      <h3 class="profile-title">{activeProfile.name}</h3>
    </div>
    
    <!-- Pagination controls (if multiple pages exist) -->
    {#if activePages.length > 1}
      <div class="pagination-controls">
        <button class="nav-arrow" onclick={prevPage} disabled={$currentPageIndex === 0}>
          ◀
        </button>
        <div class="page-dots">
          {#each activePages as _, idx (idx)}
            <span class="dot" class:active={idx === $currentPageIndex}></span>
          {/each}
        </div>
        <button class="nav-arrow" onclick={nextPage} disabled={$currentPageIndex === activePages.length - 1}>
          ▶
        </button>
      </div>
    {/if}
  </header>

  <!-- Button Key Matrix -->
  <div class="deck-grid">
    {#each gridItems as btn, index}
      {#if btn}
        <ButtonKey button={btn} />
      {:else}
        <!-- Frosted Empty Placeholder Slot -->
        <div class="key-placeholder">
          <div class="placeholder-dot"></div>
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .deck-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    justify-content: space-between;
  }

  .deck-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    padding-bottom: 10px;
    margin-bottom: 12px;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .profile-badge {
    font-size: 0.62rem;
    font-weight: 700;
    background: rgba(168, 85, 247, 0.2);
    color: #a855f7;
    padding: 3px 8px;
    border-radius: 6px;
    letter-spacing: 0.05em;
  }

  .profile-title {
    font-size: 1.05rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  /* Grid Matrix styling */
  .deck-grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr); /* 8 keys wide */
    grid-template-rows: repeat(2, 1fr);    /* 2 keys high */
    grid-gap: 12px;
    flex-grow: 1;
    align-content: center;
  }

  /* Key Placeholder Card */
  .key-placeholder {
    aspect-ratio: 1.1 / 1;
    border-radius: 16px;
    border: 1px dashed rgba(255, 255, 255, 0.02);
    background: rgba(255, 255, 255, 0.005);
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
  }

  .placeholder-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.05);
  }

  /* Navigation styling */
  .pagination-controls {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .nav-arrow {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.05);
    color: var(--text-secondary);
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 0.7rem;
    cursor: pointer;
    box-shadow: none;
    transition: all 0.2s ease;
  }

  .nav-arrow:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-primary);
  }

  .nav-arrow:disabled {
    opacity: 0.3;
    cursor: default;
  }

  .page-dots {
    display: flex;
    gap: 6px;
  }

  .dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.15);
    transition: all 0.2s ease;
  }

  .dot.active {
    background: #a855f7;
    transform: scale(1.3);
    box-shadow: 0 0 6px #a855f7;
  }
</style>
