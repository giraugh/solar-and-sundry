import z from 'zod'

export const ComicPage = z.object({
	chapter_number: z.number().positive(),
	page_number: z.number().positive(),
	image_url: z.string().url(),
	name: z.string()
})

export type ComicPage = z.infer<typeof ComicPage>
