<script>
	import TableOfContents from '$lib/components/TableOfContents.svelte';
  import TopBar from '$lib/components/TopBar.svelte'
  import SideBar from '$lib/components/SideBar.svelte'

  export let data
  export let sideBarOpen = false
</script>

<TopBar bind:sideBarOpen={sideBarOpen} />

<main>
  <slot />
</main>

<SideBar title="Table of Contents" bind:open={sideBarOpen}>
  <TableOfContents chapters={data.chapters} />
</SideBar>

<style lang="scss">
  main {
    display: flex;
    justify-content: center;
    padding-block-end: 2em;
  }
</style>
