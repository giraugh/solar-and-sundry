import z from 'zod'

import { PUBLIC_API_URL as API_URL } from '$env/static/public'
import { ComicChapter } from '$lib/schemas/chapter'

export async function load() {
	// Fetch chapters list
	const chaptersResponse = await fetch(`${API_URL}/chapter`).then((r) => r.json())
	const chapters = z.array(ComicChapter).parse(chaptersResponse)

	// Fetch page limits
	const pages = chapters.flatMap((chapter) => chapter.pages)
	const pageNumbers = pages.map((page) => page.page_number)
	const firstPage = Math.min(...pageNumbers)
	const lastPage = Math.max(...pageNumbers)
	const pageLimits = { first: firstPage, last: lastPage }

	return {
		chapters,
		pages,
		pageLimits
	}
}
