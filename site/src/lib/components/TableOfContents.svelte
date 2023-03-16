<script lang="ts">
	import dayjs from 'dayjs'

	import type { ComicChapter } from '$lib/schemas/chapter'
	import pageNumberStore from '$lib/stores/pageNumber'
	import { Star } from 'lucide-svelte'

	export let chapters: ComicChapter[]
  export let showLastRead = false
  export let showDate = false
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
						{#if showLastRead && $pageNumberStore === page.page_number}
							<em class='last-read-mark'>
								<span class="star-icon"><Star size="13" /></span>
								<span class="last-read-label">(last read)</span>
							</em>
						{/if}
						{#if showDate}
							<em class="date-label">({dayjs(page.published_at).format('YYYY/MM/DD')})</em>
						{/if}
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

	.date-label {
		float: right;
		opacity: .75;

		@media (max-width: 500px) {
			 display: none;
		}
	}

	ol {
		list-style-type: none;
		padding-inline-start: 0em;
	}

	ol.page-list {
		padding-block: 0.5em;
		padding-inline-start: 1em;

		li {
			margin-block-end: 0.4em;

			a {
				color: inherit;
			}
		}

		@media (max-width: 500px) {
			padding-inline-start: .5em;
		}
	}

	.last-read-mark {

		.star-icon { display: none; }

		@media (max-width: 500px) {
			.last-read-label { display: none; }	
			.star-icon { display: initial; }
		}
	}
</style>
