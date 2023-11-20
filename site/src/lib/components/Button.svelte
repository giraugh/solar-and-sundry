<script lang="ts">
	export let disabled = false
	export let href: string | undefined = undefined
	export let square = false

	let className = ''
	export { className as class }
</script>

{#if href}
	{#if !disabled}
		<a class="{className} button" {href} data-sveltekit-preload-data="hover">
			<slot />
		</a>
	{:else}
		<span class="{className} button"><slot /></span>
	{/if}
{:else}
	<button type="button" class="{className} button" class:square {disabled} on:click>
		<slot />
	</button>
{/if}

<style>
	button,
	a,
	span {
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--col-surface-deco);
		font-weight: 700;
		font-family: var(--font-body);
		border-radius: var(---radius-2);
		box-shadow: inset 0 1px 2px rgb(200 200 255 / 0.2), inset 0 -2px 1px rgb(20 0 100 / 0.3);
	}

	button:active,
	a:active {
		box-shadow: inset 0 -3px 2px #c8c8ff33, inset 0 1px 2px #1400644d;
		transform: translateY(1px);
	}

	button {
		border: none;
		cursor: pointer;

		&.square {
			aspect-ratio: 1;
		}
	}

	a {
		text-align: center;
		color: inherit;
		padding-block: 1em;
		padding-inline: 0.5em;
	}
</style>
