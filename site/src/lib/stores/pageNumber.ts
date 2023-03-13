import { browser } from '$app/environment'
import { writable } from 'svelte/store'

const PAGE_NUM_STORAGE_KEY = 'solar-and-sundry-page-number'
const storedPageNumber = browser ? Number(localStorage.getItem(PAGE_NUM_STORAGE_KEY) ?? '0') : 0

// Set stored value
const pageNumberStore = writable<number>(storedPageNumber)

// Any time store changes, write to local storage
pageNumberStore.subscribe(
	(pageNumber) => browser && localStorage.setItem(PAGE_NUM_STORAGE_KEY, String(pageNumber))
)

export default pageNumberStore
