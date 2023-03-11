<script lang="ts">
	import type { ComicChapter } from '$lib/schemas/chapter';

  export let chapters: ComicChapter[]
</script>

<ol>
  {#each chapters as chapter}
    <div class="chapter">
      Chapter {chapter.chapter_number}
      <ol>
        {#each chapter.pages as page}
          <li>Page {page.page_number}</li>
        {/each}
      </ol>
    </div>
  {/each}
</ol>

<style lang="scss">
  ol {
    list-style-type: none;
    padding-inline-start: 1em;
  }
</style>
