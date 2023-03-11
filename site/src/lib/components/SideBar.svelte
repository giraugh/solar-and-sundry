<script lang="ts">
	import { fly, fade } from 'svelte/transition'
  import { X } from 'lucide-svelte'

  export let title = "Sidebar";
  export let open = false

  let close = () => { open = false }
</script>

{#if open}
  <nav transition:fly="{{ x: 300, duration: 300 }}">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="scrim"
      on:click={close}
      transition:fade
    />
    <div class="content">
      <div class="header">
        <h2>{title}</h2>
        <button class="close-btn" on:click={close}><X /></button>
      </div>
      <slot />
    </div>
  </nav>
{/if}
<svelte:window on:keyup={e => e.key === 'Escape' && open ? close() : null}></svelte:window>

<style lang="scss">
  nav {
    position: fixed;
    min-width: 20em;
    height: 100vh;
    right: 0;
    top: 0;
    box-sizing: border-box;

    .scrim {
      position: fixed;
      inset: 0;
      background: #5d5d5d8f;
    }

    .content {
      position: absolute;
      inset: 0;
      background: var(--col-surface-alt);
      color: var(--col-text-surface);
      padding: .5em 1.25em;
      z-index: 2;
    }
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-block-end: 1em;
    padding-block: .75em;
    padding-inline: .5em;
    border-bottom: 1px solid var(--col-surface);
  
    h2 { margin: 0; }
  
    button {
      display: flex;
      justify-content: center;
      align-items: center;
      background: var(--col-surface-deco);
      border-radius: .3rem;
      padding: .2em;
      aspect-ratio: 1;
      outline: none;
      border: none;
      font-weight: bold;
    }
  }
</style>
