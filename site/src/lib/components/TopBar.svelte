<script>
	import { BookOpen, Menu } from 'lucide-svelte'
	import discordLogo from '$lib/images/discord-mark-white.svg'

	export let sideBarOpen = false
</script>

<nav class="topbar">
	<a href="/">
		<img
			class="logo"
			src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/6b7613e1-2834-4db0-ca26-f03b2e36df00/public"
			alt="Solar and Sundry Logo"
		/>
		<h1>Solar and Sundry</h1>
	</a>
	<div class="spacer" />
	<a title="discord server" href="https://discord.gg/gnDrvjWnWD" target="_blank"
		><img class="discord-logo" width="24" height="24" src={discordLogo} alt="discord logo" /></a
	>
	<button title="table of contents" class="sidebar-btn" on:click={() => (sideBarOpen = true)}
		><BookOpen /></button
	>
</nav>

<style>
	nav.topbar {
		display: flex;
		gap: 1em;
		padding-block: 0.25em;
		padding-inline: 0.75em;
		align-items: center;
		min-height: 4em;
		color: var(--col-surface);

		& * {
			margin: 0;
		}

		& a {
			text-decoration: none;
			color: inherit;
		}

		& a:first-of-type {
			display: flex;
			align-items: center;
			gap: 1em;
		}
	}

	@media (max-width: 600px) {
		nav.topbar {
			/* background: var(--col-surface-alt); */
			/* color: var(--col-text-surface); */
		}
	}

	.discord-logo {
		filter: brightness(0) saturate(100%) invert(75%) sepia(6%) saturate(6963%) hue-rotate(194deg)
			brightness(93%) contrast(90%);
	}

	.spacer {
		flex: 1;
	}

	button {
		background: none;
		border: none;
		color: inherit;
	}

	.logo {
		display: block;
		background: var(--col-background);
		border-radius: 100vmax;
		aspect-ratio: 1;
		flex: 1;
		max-width: 3em;
		border: 2px solid #5f69b2;
	}
</style>
