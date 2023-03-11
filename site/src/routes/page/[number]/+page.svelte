<script lang="ts">
	import { goto } from '$app/navigation';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
  import { page as pageStore } from '$app/stores'
  import z from 'zod'
	import type { ComicPage } from '$lib/schemas/page';
	import { fade, fly, slide } from 'svelte/transition';

  export let data;

  $: pageNumber = z.number({ coerce: true }).parse($pageStore.params.number)
  $: hasNextPage = pageNumber !== data.pageLimits.last
  $: hasPreviousPage = pageNumber !== data.pageLimits.first

  let page: ComicPage
  $: {
    const pageMaybe = data.pages.find(page => page.page_number === pageNumber)
    if (!pageMaybe) throw new Error(`No such page with page number ${pageNumber}`)
    page = pageMaybe
  }

  // Track image loading state
  let imageLoaded = false
  const clearImageLoaded = (_: unknown) => { imageLoaded = false }
</script>

<div class="comic-wrapper">
  <p class="comic-label">ch{page.chapter_number} | p{page.page_number}</p>
  <div class="image-wrapper">
    <p class="huge-text">{page.page_number}</p>
    {#key pageNumber}
      <img
        in:fade="{{ duration: 400, delay: 200 }}"
        use:clearImageLoaded
        on:load={() => imageLoaded = true}
        class:loading={!imageLoaded}
        class="comic"
        src="{page.image_url}"
        alt="comic page {page.page_number}" >
    {/key}
  </div>
  <div class="controls">
    {#if hasPreviousPage}
      <a data-sveltekit-preload-data="hover" href="{String(page.page_number-1)}"><ChevronLeft /></a>
    {:else}
      <span><ChevronLeft /></span>
    {/if}
    {#if hasNextPage}
      <a data-sveltekit-preload-data="hover" href="{String(page.page_number+1)}"><ChevronRight /></a>
    {:else}
      <span><ChevronRight /></span>
    {/if}
  </div>
</div>

<style lang="scss">
  .comic-wrapper {
    display: flex;
    flex-direction: column;
    padding: 1em;
    width: 100%;
    max-width: 40em;
  }

  .image-wrapper {
    position: relative;
    aspect-ratio: 1601 / 2561;
    background: var(--col-surface-alt);

    .huge-text {
      position: absolute;
      inset: 0;
      display: flex;
      justify-content: center;
      align-items: center;
      text-align: center;
      font-size: 150px;
      font-weight: bold;
      color: black;
      opacity: .2;
    }

    img {
      /* Fade In */
      transition: opacity .4s;
      &.loading { opacity: 0; }

      position: absolute;
      inset: 0;
    }
  }

  .comic-label {
    width: max-content;
    align-self: flex-end;
    margin-block: 0 .5em;
  }

  img.comic {
    display: block;
    width: 100%;
  }

  .controls {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: .5em;
    margin-block-start: .5em;

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
