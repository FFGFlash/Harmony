<script lang="ts">
	import type { FullProfile } from '$lib/types'
	import type { Snippet } from 'svelte'
	import { twMerge } from 'tailwind-merge'

	interface Props {
		children?: Snippet
		profile: FullProfile
		class?: string
	}

	const { profile, children, class: klass }: Props = $props()

	const name = $derived(profile.display_name || profile.username)

	const className = $derived(
		twMerge('btn flex items-center justify-start gap-2 px-2 hover:preset-tonal', klass)
	)
</script>

<button class={className}>
	<div class="flex size-8 items-center justify-center rounded-full bg-secondary-500">
		{name.slice(0, 2).toUpperCase()}
	</div>
	<div class="flex flex-col">
		<span>{name}</span>
		<span>{profile.status_emoji} {profile.custom_status}</span>
	</div>
	{@render children?.()}
</button>
