<script lang="ts">
	import { ChevronFirst, ChevronLast, ChevronLeft, ChevronRight } from 'lucide-svelte'

	export let pageNumber: number
	export let pageLimits: { first: number; last: number }

	$: hasNextPage = pageNumber !== pageLimits.last
	$: hasPreviousPage = pageNumber !== pageLimits.first
</script>

<div class="controls">
	<!-- First -->
	{#if pageNumber !== pageLimits.first}
		<a title="Oldest page" data-sveltekit-preload-data="hover" href={String(pageLimits.first)}
			><ChevronFirst /></a
		>
	{:else}
		<span><ChevronFirst /></span>
	{/if}

	<!-- Previous -->
	{#if hasPreviousPage}
		<a title="Previous page" data-sveltekit-preload-data="hover" href={String(pageNumber - 1)}
			><ChevronLeft /></a
		>
	{:else}
		<span><ChevronLeft /></span>
	{/if}

	<!-- Next -->
	{#if hasNextPage}
		<a title="Next page" data-sveltekit-preload-data="hover" href={String(pageNumber + 1)}
			><ChevronRight /></a
		>
	{:else}
		<span><ChevronRight /></span>
	{/if}

	<!-- Last -->
	{#if pageNumber !== pageLimits.last}
		<a title="Newest page" data-sveltekit-preload-data="hover" href={String(pageLimits.last)}
			><ChevronLast /></a
		>
	{:else}
		<span><ChevronLast /></span>
	{/if}
</div>

<style lang="scss">
	.controls {
		display: grid;
		grid-template-columns: 3em 1fr 1fr 3em; // repeat(4, 1fr);
		gap: 0.5em;
		margin-block: 0.5em;

		a,
		span {
			display: flex;
			align-items: center;
			justify-content: center;
			background: var(--col-surface);
			border: none;
			padding-block: 0.5em;
			color: var(--col-background);
			cursor: pointer;
			border-radius: var(--border-radius-button);
			box-shadow:
				inset 0 1px 2px rgb(200 200 255 / .2),
				inset 0 -2px 1px rgb(20 0 100 / .3);
		}
		
		a:active {
			box-shadow: inset 0 -3px 2px #c8c8ff33, inset 0 1px 2px #1400644d;
			transform: translateY(1px);
		}

		span {
			opacity: 0.5;
			cursor: unset;
		}
	}
</style>
