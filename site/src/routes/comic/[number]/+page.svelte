<script lang="ts">
  import { page as pageStore } from '$app/stores'
  import z from 'zod'
	import type { ComicPage } from '$lib/schemas/page';
	import { fade } from 'svelte/transition';
	import ComicControls from '$lib/components/ComicControls.svelte';

  export let data

  $: pageNumber = z.number({ coerce: true }).parse($pageStore.params.number)

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

<svelte:head>
  <title>SaS | Page {pageNumber}</title>
</svelte:head>

<section class="comic-section">
  <div class="comic-labels">
    <p class="comic-name">
      {page.name}
    </p>
    <p class="comic-number">
      ch{page.chapter_number} | p{page.page_number}
    </p>
  </div>
  <ComicControls pageNumber={pageNumber} pageLimits={data.pageLimits} />
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
  <ComicControls pageNumber={pageNumber} pageLimits={data.pageLimits} />
</section>

<style lang="scss">
  .comic-section {
    justify-self: center;
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

  .comic-labels {
    display: flex;
    justify-content: space-between;
    /* width: max-content; */
    margin-block: 0 .1em;
  
    p { 
      margin-block: 0;
    }
  }

  img.comic {
    display: block;
    width: 100%;
  }
</style>