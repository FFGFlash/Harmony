<script lang="ts">
	import LoadingSpinner from './LoadingSpinner.svelte'
	import { api } from '$lib/api.svelte'
	import { createQuery } from '@tanstack/svelte-query'
	import { channelHistory } from '$lib/stores/channelHistory.svelte'

	const serversQuery = createQuery(() => ({
		queryKey: ['servers'],
		queryFn: () => api.getServers()
	}))

	function getServerLink(serverId: string, mainChannelId?: string | null) {
		const lastChannelId = channelHistory.getLastChannel(serverId, mainChannelId)
		return `/channels/${serverId}/${lastChannelId}`
	}
</script>

{#if serversQuery.isPending}
	<div class="btn-icon size-12 p-0">
		<LoadingSpinner class="size-6" />
	</div>
{:else if serversQuery.error}
	<div class="btn-icon size-12 preset-filled-error-600-400 p-0">ER</div>
{:else}
	{#each serversQuery.data as server}
		<a
			href={getServerLink(server.id, server.main_channel_id)}
			class="btn-icon size-12 rounded-[50%] preset-filled-secondary-500 p-0 transition-all hover:rounded-2xl"
		>
			{server.name.slice(0, 2).toUpperCase()}
		</a>
	{/each}
{/if}
