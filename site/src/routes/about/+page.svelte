<script lang="ts">
	import TableOfContents from '$lib/components/TableOfContents.svelte';


  export let data
</script>

<div class="wrapper">
  <img class="banner-img" src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/70978e70-85d7-448a-e90e-5bef7e311800/public" alt="page wide panel from solar and sundry">
  <section class="description">
    <div class="left">
      <h2>"Rejuvenate and Prepare"</h2>
      <p>
        Welcome to the home of <em>Solar and Sundry</em>, a sci-fi webcomic about creating an ecosystem where one shouldn&apos;t be.
      </p>
      <p>New pages every month.</p>
    </div>
    <div class="right">
      <img src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/155993e5-539b-4c51-29ee-a9c3894bd400/public" alt="panel from solar and sundry p1" />
      <img src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/4b15df30-85c7-429e-d062-bf1d19f0fd00/public" alt="panel from solar and sundry p2" />
    </div>
  </section>
  <section class="buttons">
    <!-- <a class="wide" href="#">Continue reading</a> -->
    <a href="/comic/{data.pageLimits.first}">Start at the beginning</a>
    <a href="/comic/{data.pageLimits.last}">Read newest page</a>
  </section>
  <section class="recent-pages">
    <h2>Archive</h2>
    <TableOfContents chapters={data.chapters} />
  </section>
</div>

<style lang="scss">
  .wrapper {
    max-width: 40em;
    background: var(--col-surface-alt);
    color: var(--col-text-surface);
    padding: 1.5em;
    margin-block-start: 2em;

    h2:first-of-type {
      margin-block-start: 0;
    }

    section {
      margin-block: 1em;

      &:first-of-type {
        margin-block-start: 0;
      }
    }

    @media (max-width: 600px) {
      margin-block-start: 0;
      box-shadow: inset -1px 5px 10px -4px #31224068;

      .description {
        h2 { display: none; }
        p:first-of-type { margin-block-start: 0; }
        .right {
          display: none;
        }
      }
    }
  }

  .banner-img {
    @media (min-width: 600px) {
      display: none;
    }

    display: block;
    object-fit: cover;
    width: 100%;
    max-height: 8em;
    margin-block-end: 1em;
  }

  .description {
    display: flex;
    gap: 1.5em;
  
    .right {
      display: flex;
      gap: .5em;

      img {
        min-width: 7em;
        width: 100%;
        object-fit: cover;
        object-position: center;
        height: 100%;
        background: var(--col-surface-deco);
      }
    }
  }

  .buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: .5em;

    a.wide {
      grid-column: 1 / 3;
    }

    a {
      display: block;
      text-align: center;
      color: inherit;
      padding-block: 1em;
      padding-inline: .5em;
      background: var(--col-surface-deco);
      border: none;
      outline: none;
      font-weight: bold;
      font-family: var(--font-body);
    }
  }

  section.buttons {
    margin-block-end: 3em;
  }
</style>
