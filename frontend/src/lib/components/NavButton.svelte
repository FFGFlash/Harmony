<script lang="ts">
	import { page } from '$app/state'
	import type { Snippet } from 'svelte'
	import { twMerge } from 'tailwind-merge'

	export interface Props {
		children: Snippet
		href: string
		exact?: boolean
		exclude?: string[]
		class?: string
	}

	const { children, href, class: propClassName, exact, exclude }: Props = $props()

	const className = $derived(twMerge('', propClassName))

	const active = $derived(
		exact
			? href === page.url.pathname
			: page.url.pathname.startsWith(href) &&
					(!exclude || !exclude.some((ex) => page.url.pathname.startsWith(ex)))
	)
</script>

<a {href} class={className} data-active={active ? '' : undefined}>
	{@render children()}
</a>
