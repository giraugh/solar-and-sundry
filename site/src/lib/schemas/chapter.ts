import z from 'zod'
import { ComicPage } from './page'

export const ComicChapter = z.object({
	chapter_number: z.number().positive(),
	pages: z.array(ComicPage)
})

export type ComicChapter = z.infer<typeof ComicChapter>
