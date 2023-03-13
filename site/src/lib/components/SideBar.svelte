<script lang="ts">
	import { fly, fade } from 'svelte/transition'
	import { X } from 'lucide-svelte'
	import Button from './Button.svelte'

	export let title = 'Sidebar'
	export let open = false

	let close = () => {
		open = false
	}
</script>

{#if open}
	<nav transition:fly={{ x: 300, duration: 300 }}>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div class="scrim" on:click={close} />
		<div class="content">
			<div class="header">
				<h2>{title}</h2>
				<Button square on:click={close}><X /></Button>
			</div>
			<slot />
		</div>
	</nav>
{/if}

<svelte:window on:keyup={(e) => (e.key === 'Escape' && open ? close() : null)} />

<style lang="scss">
	nav {
		position: fixed;
		min-width: 20em;
		height: 100vh;
		right: 0;
		top: 0;
		box-sizing: border-box;
		--scrim-col: rgb(56 53 86 / 53%);

		.scrim {
			position: fixed;
			inset: 0;
		}

		.content {
			position: absolute;
			inset: 0;
			background: var(--col-surface-alt);
			color: var(--col-text-surface);
			padding: 0.5em 1.25em;
			z-index: 2;
			box-shadow: 0px 0px 0px 1000px var(--scrim-col);
		}
	}

	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 1em;
		padding-block: 0.75em;
		padding-inline: 0.5em;
		border-bottom: 2px solid var(--col-surface-deco);

		h2 {
			margin: 0;
		}
	}
</style>
