<script lang="ts">
	import { ChevronFirst, ChevronLast, ChevronLeft, ChevronRight } from 'lucide-svelte';

  export let pageNumber: number
  export let pageLimits: { first: number, last: number }

  $: hasNextPage = pageNumber !== pageLimits.last
  $: hasPreviousPage = pageNumber !== pageLimits.first
</script>

<div class="controls">
  <!-- First -->
  {#if pageNumber !== pageLimits.first}
    <a title="Oldest page" data-sveltekit-preload-data="hover" href="{String(pageLimits.first)}"><ChevronFirst /></a>
  {:else}
    <span><ChevronFirst /></span>
  {/if}

  <!-- Previous -->
  {#if hasPreviousPage}
    <a title="Previous page" data-sveltekit-preload-data="hover" href="{String(pageNumber-1)}"><ChevronLeft /></a>
  {:else}
    <span><ChevronLeft /></span>
  {/if}

  <!-- Next -->
  {#if hasNextPage}
    <a title="Next page" data-sveltekit-preload-data="hover" href="{String(pageNumber+1)}"><ChevronRight /></a>
  {:else}
    <span><ChevronRight /></span>
  {/if}

  <!-- Last -->
  {#if pageNumber !== pageLimits.last}
    <a title="Newest page" data-sveltekit-preload-data="hover" href="{String(pageLimits.last)}"><ChevronLast /></a>
  {:else}
    <span><ChevronLast /></span>
  {/if}

</div>


<style lang="scss">
  .controls {
    display: grid;
    grid-template-columns: 3em 1fr 1fr 3em; // repeat(4, 1fr);
    gap: .5em;
    margin-block: .5em;

    a, span {
      display: flex;
      align-items: center;
      justify-content: center;
      background: var(--col-surface);
      border: none;
      outline: none;
      padding-block: .5em;
      color: var(--col-background);
      cursor: pointer;
    }

    span {
      opacity: .5;
      cursor: unset;
    }
  }
</style>
