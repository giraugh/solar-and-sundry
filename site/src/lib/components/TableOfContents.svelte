<script lang="ts">
	import type { ComicChapter } from '$lib/schemas/chapter';

  export let chapters: ComicChapter[]
</script>

<ol>
  {#each chapters as chapter}
    <div class="chapter">
      <strong class="chapter-name">Chapter {chapter.chapter_number}</strong>
      <ol class="page-list">
        {#each chapter.pages as page}
          <li>
            <a href="/comic/{page.page_number}">
              p{page.page_number}
              &mdash;
              {page.name}
            </a>
          </li>
        {/each}
      </ol>
    </div>
  {/each}
</ol>

<style lang="scss">
  
  .chapter {
    margin-block-end: 1em;
  }
  
  .chapter-name {
    display: block;
    font-weight: bold;
  }

  ol {
    list-style-type: none;
    padding-inline-start: 0em;
  }
  
  ol.page-list {
    padding-block: .5em;
    padding-inline-start: 1em;

    li {
      margin-block-end: .4em;

      a {
        color: inherit;
      }
    }
  }
</style>
