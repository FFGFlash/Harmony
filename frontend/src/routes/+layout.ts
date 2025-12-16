import { QueryClient } from '@tanstack/svelte-query'
import type { LayoutLoad } from './$types'
import { browser } from '$app/environment'
import { auth } from '$lib/stores/auth.svelte'

export const ssr = false

export const load: LayoutLoad = async () => {
	await auth.initialize()

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser,
				staleTime: 60 * 1000
			}
		}
	})

	return { queryClient }
}
