<script lang="ts">
	import { createInfiniteQuery } from '@tanstack/svelte-query'
	import { api } from '$lib/api.svelte'
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte'
	import UserCard from '$lib/components/UserCard.svelte'

	const friends = createInfiniteQuery(() => ({
		queryKey: ['friends'],
		initialPageParam: 0,
		queryFn({ pageParam }) {
			return api.getFriends(pageParam)
		},
		getNextPageParam(lastPage) {
			return lastPage.has_more ? lastPage.offset + lastPage.limit : undefined
		},
		select(data) {
			return data.pages.flatMap((p) => p.data)
		}
	}))
</script>

<div class="flex h-full flex-col">
	<div class="border-b border-surface-300-700 px-6 py-4">
		<h2 class="text-xl font-semibold">Friends</h2>
		<p class="text-sm text-surface-600-400">
			{friends.data?.length ?? 0} friend{friends.data?.length === 1 ? '' : 's'}
		</p>
	</div>

	<div class="flex-1 overflow-y-auto p-4">
		{#if friends.isPending}
			<div class="flex h-full items-center justify-center">
				<LoadingSpinner class="size-12" />
			</div>
		{:else if friends.isError}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<p class="text-error-600-400">Failed to load friends</p>
					<p class="mt-2 text-sm text-surface-600-400">{friends.error.message}</p>
				</div>
			</div>
		{:else if friends.data.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<p class="text-surface-600-400">No friends yet</p>
					<p class="mt-2 text-sm text-surface-600-400">Add friends to see them here</p>
				</div>
			</div>
		{:else}
			<div class="grid gap-2">
				{#each friends.data as friend (friend.id)}
					<UserCard profile={friend} class="w-full" />
				{/each}
			</div>
			{#if friends.hasNextPage}
				<button
					onclick={() => friends.fetchNextPage()}
					disabled={friends.isFetchingNextPage}
					class="mt-4 btn w-full preset-tonal"
				>
					{friends.isFetchingNextPage ? 'Loading...' : 'Load More'}
				</button>
			{/if}
		{/if}
	</div>
</div>
