<script lang="ts">
	import TableOfContents from '$lib/components/TableOfContents.svelte'
  import Button from '$lib/components/Button.svelte'

  export let data
</script>

<div class="wrapper">
  <img class="banner-img" src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/70978e70-85d7-448a-e90e-5bef7e311800/public" alt="page wide panel from solar and sundry">
  <section class="description">
    <div class="left">
      <!-- <h2>Solar and Sundry</h2> -->
      <p>
        Welcome to the home of <em>Solar and Sundry</em>, a sci-fi webcomic about creating an ecosystem where one shouldn&apos;t be.
      </p>
      <p>New pages every month.</p>
    </div>
    <!-- <div class="right">
      <img src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/155993e5-539b-4c51-29ee-a9c3894bd400/public" alt="panel from solar and sundry p1" />
      <img src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/4b15df30-85c7-429e-d062-bf1d19f0fd00/public" alt="panel from solar and sundry p2" />
    </div> -->
  </section>
  <section class="buttons">
    <!-- <a class="wide" href="#">Continue reading</a> -->
    <Button href="/comic/{data.pageLimits.first}">Start at the beginning</Button>
    <Button href="/comic/{data.pageLimits.last}">Read newest page</Button>
  </section>
  <section class="recent-pages">
    <h2>Archive</h2>
    <TableOfContents chapters={data.chapters} />
  </section>
</div>

<svelte:head>
  <title>Solar and Sundry</title>
</svelte:head>

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

      .banner-img {
        margin-block-end: 1em;
      }

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
    display: block;
    object-fit: cover;
    width: 100%;
    max-height: 10em;
    margin-block-end: .5em;
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

    .button.wide {
      grid-column: 1 / 3;
    }
  }

  section.buttons {
    margin-block-end: 3em;
  }
</style>
