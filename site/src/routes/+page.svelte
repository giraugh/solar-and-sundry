<script lang="ts">
	import TableOfContents from '$lib/components/TableOfContents.svelte'
	import Button from '$lib/components/Button.svelte'
	import pageNumberStore from '$lib/stores/pageNumber'

	export let data
</script>

<div class="wrapper">
	<img
		class="banner-img"
		src="https://imagedelivery.net/zthi1l8fKrUGB5ig08mq-Q/de292ba7-f164-4f43-ec17-1876a7a44600/public"
		alt=""
	/>
	<div class="content">
		<section class="description">
			<div class="left">
				<!-- <h2>Solar and Sundry</h2> -->
				<p>
					Welcome to the home of <em>Solar and Sundry</em>, a sci-fi horror webcomic about life
					blooming against all odds.
				</p>
				<p><em>New pages every month.</em></p>
			</div>
		</section>
		<section class="buttons">
			{#if $pageNumberStore > 0}
				<div class="wide">
					<Button href="/comic/{$pageNumberStore}">Continue from p{$pageNumberStore}</Button>
				</div>
			{/if}
			<Button href="/comic/{data.pageLimits.first}">Start at the beginning</Button>
			<Button href="/comic/{data.pageLimits.last}">Read newest page</Button>
		</section>
		<section class="recent-pages">
			<h2>Archive</h2>
			<TableOfContents showLastRead showDate chapters={data.chapters} />
		</section>
	</div>
</div>

<svelte:head>
	<title>Solar and Sundry</title>
</svelte:head>

<style lang="scss">
	.wrapper {
		max-width: 40em;
		background: var(--col-surface-alt);
		color: var(--col-text-surface);
		margin-block-start: 2em;

		@media (min-width: 600px) {
			border-radius: 0.15rem;
		}

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
				h2 {
					display: none;
				}
				p:first-of-type {
					margin-block-start: 0;
				}
			}
		}
	}

	.content {
		padding: 1.5em;
		padding-block-start: 0.5em;
	}

	.banner-img {
		display: block;
		object-fit: cover;
		width: 100%;
		max-height: 25em;
		margin-block-end: 0.5em;

		@media (max-width: 600px) {
			max-height: 17em;
		}
	}

	.buttons {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.5em;

		.wide {
			grid-column: 1 / 3;
		}
	}

	section.buttons {
		margin-block-end: 3em;
	}
</style>
