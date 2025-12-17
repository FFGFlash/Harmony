<script lang="ts">
	import { createInfiniteQuery, createQuery } from '@tanstack/svelte-query'
	import { api } from '$lib/api.svelte'
	import UserCard from './UserCard.svelte'

	interface Props {
		serverId: string
	}

	const { serverId }: Props = $props()

	const members = createInfiniteQuery(() => ({
		queryKey: ['servers', serverId, 'members'],
		initialPageParam: 0,
		queryFn({ pageParam }) {
			return api.getServerMembers(serverId, pageParam)
		},
		getNextPageParam(lastPage) {
			return lastPage.has_more ? lastPage.offset + lastPage.offset : undefined
		},
		select(data) {
			return data.pages.flatMap((p) => p.data)
		}
	}))
</script>

<div class="flex h-dvh w-60 flex-col overflow-y-auto border-l border-surface-300-700 px-4 py-2">
	{#if members.isPending}
		Loading...
	{:else if members.isError}
		Error
	{:else}
		{#each members.data as member}
			<UserCard profile={member} />
		{/each}
	{/if}
</div>
