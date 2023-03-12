<script>
	import Sidebar from '$lib/components/SideBar.svelte'
  import { Menu } from 'lucide-svelte'

  export let sideBarOpen = false
</script>

<nav class="topbar">
  <img class="logo" src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/6b7613e1-2834-4db0-ca26-f03b2e36df00/public" alt="Solar and Sundry Logo" />
  <h1>Solar and Sundry</h1>
  <div class="spacer" />
  <button class="sidebar-btn" on:click={() => sideBarOpen = true}><Menu /></button>
</nav>

<style lang="scss">
  nav.topbar {
    display: flex;
    gap: 1em;
    padding-inline: .75em;
    align-items: center;
    background: var(--col-surface);
    min-height: 4em;
    color: var(--col-background);
  
    * { margin: 0; }
  }

  .spacer {
    flex: 1;
  }

  .sidebar-btn {
    background: none;
    border: none;
    outline: none;
    color: inherit;
  }

  .logo {
    display: block;
    background: var(--col-background);
    border-radius: 100vmax;
    aspect-ratio: 1;
    flex: 1;
    max-width: 3em;
  }
</style>
