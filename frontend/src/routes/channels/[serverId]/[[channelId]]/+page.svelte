<script lang="ts">
	import type { PageProps } from './$types'
	import { api } from '$lib/api.svelte'
	import { createQuery } from '@tanstack/svelte-query'
	import { channelHistory } from '$lib/stores/channelHistory.svelte'
	import { untrack } from 'svelte'
	import { websocket } from '$lib/stores/websocket.svelte'
	import ChannelView from '$lib/components/ChannelView.svelte'
	import MemberList from '$lib/components/MemberList.svelte'

	const { params, data }: PageProps = $props()

	const serverId = $derived(params.serverId)
	const channelId = $derived(data.channelId)

	const channels = createQuery(() => ({
		queryKey: ['servers', serverId, 'channels'],
		queryFn: () => api.getServerChannels(serverId)
	}))

	const channel = $derived(
		channelId != null ? channels.data?.find((ch) => ch.id === channelId) : undefined
	)

	$effect(() => {
		if (!serverId || !channelId) return
		untrack(() => channelHistory.setLastChannel(serverId, channelId))
	})
</script>

<div class="flex h-dvh">
	{#if channels.isPending}
		<div>Loading...</div>
	{:else if channels.isError}
		<div>Error</div>
	{:else if channel}
		<ChannelView {channel} />
	{:else}
		<div class="flex h-dvh flex-1 items-center justify-center">Not Found</div>
	{/if}
	<MemberList {serverId} />
</div>
