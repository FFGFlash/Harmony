<script lang="ts">
	import { page } from '$app/state'
	import { channelHistory } from '$lib/stores/channelHistory.svelte'
	import { untrack } from 'svelte'

	const { children } = $props()

	$effect(() => {
		const channelId = page.params.channelId
		untrack(() => channelHistory.setLastChannel('@me', channelId))
	})
</script>

{@render children()}
