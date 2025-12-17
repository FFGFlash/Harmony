<script lang="ts">
	import { page } from '$app/state'
	import { api } from '$lib/api.svelte'
	import ChannelIcon from '$lib/components/ChannelIcon.svelte'
	import ChannelLink from '$lib/components/ChannelLink.svelte'
	import NavButton from '$lib/components/NavButton.svelte'
	import ServerList from '$lib/components/ServerList.svelte'
	import UserSearch from '$lib/components/UserSearch.svelte'
	import { channelHistory } from '$lib/stores/channelHistory.svelte'

	import { MessageCircleHeartIcon, UsersIcon, HashIcon, AudioLinesIcon } from '@lucide/svelte'
	import { createQuery } from '@tanstack/svelte-query'

	let { children } = $props()

	const segments = $derived(page.params)
	const isDM = $derived(page.url.pathname.includes('/@me'))

	const server = createQuery(() => ({
		queryKey: ['servers', page.params.serverId],
		enabled: page.params.serverId != null,
		queryFn: () => api.getServer(page.params.serverId!)
	}))

	const dmChannels = createQuery(() => ({
		queryKey: ['dms'],
		enabled: page.params.serverId == null,
		queryFn: () => api.getDMs()
	}))

	const serverChannels = createQuery(() => ({
		queryKey: ['servers', page.params.serverId, 'channels'],
		enabled: page.params.serverId != null,
		queryFn: () => api.getServerChannels(page.params.serverId!)
	}))
</script>

<div class="flex h-dvh overflow-hidden bg-surface-200-800">
	<div
		class="scrollbar-none flex w-18 flex-col items-center space-y-2 overflow-y-scroll bg-surface-50-950 py-3"
	>
		<a
			href={`/channels/@me/${channelHistory.getLastChannel('@me')}`}
			class="btn-icon size-12 rounded-[50%] preset-filled-primary-500 p-0 transition-all hover:rounded-2xl"
		>
			<MessageCircleHeartIcon class="size-6" />
		</a>

		<hr class="hr" />

		<ServerList />
	</div>

	<div class="flex w-60 flex-col bg-surface-100-900">
		{#if isDM}
			<div class="flex h-12 items-center border-b border-surface-500 px-2 shadow-sm">
				<UserSearch />
			</div>
		{:else}
			<div class="flex h-12 items-center border-b border-surface-500 px-4 shadow-sm">
				<h2 class="truncate h2 text-base">
					{server.data?.name}
				</h2>
			</div>
		{/if}

		<div class="flex-1 overflow-y-auto pt-4">
			{#if isDM}
				<NavButton
					href="/channels/@me"
					exact
					class="mx-2 btn flex justify-start px-2 hover:preset-tonal data-active:preset-tonal"
				>
					<UsersIcon class="size-5" /> Friends
				</NavButton>

				<div class="mt-4 px-2">
					<div class="mb-2 flex items-center justify-between px-2">
						<span class="text-xs font-semibold text-surface-600-400 uppercase">Direct Messages</span
						>
					</div>

					{#if dmChannels.isPending}
						<div>Loading...</div>
					{:else if dmChannels.isError}
						<div>Error</div>
					{:else}
						{#each dmChannels.data as channel}
							<NavButton
								href={`/channels/@me/${channel.id}`}
								class="btn flex justify-start px-2 hover:preset-tonal data-active:preset-tonal"
							>
								<div class="size-8 rounded-full">
									{channel.name.slice(0, 2).toUpperCase()}
								</div>
								<span class="truncate">{channel.name}</span>
							</NavButton>
						{/each}
					{/if}
				</div>
			{:else}
				<div class="px-2">
					<div class="mb-2 flex items-center justify-between px-2">
						<span class="text-xs font-semibold text-surface-600-400 uppercase">Channels</span>
					</div>

					{#if serverChannels.isPending}
						<div>Loading...</div>
					{:else if serverChannels.isError}
						<div>Error</div>
					{:else}
						{#each serverChannels.data as channel}
							<NavButton
								href={`/channels/${channel.server_id}/${channel.id}`}
								class="btn flex justify-start px-2 hover:preset-tonal data-active:preset-tonal"
							>
								<span>
									<ChannelIcon type={channel.channel_type} class="size-4" />
								</span>
								<span class="truncate">{channel.name}</span>
							</NavButton>
						{/each}
					{/if}
				</div>
			{/if}
		</div>
	</div>

	<div class="flex-1">
		{@render children()}
	</div>
</div>
