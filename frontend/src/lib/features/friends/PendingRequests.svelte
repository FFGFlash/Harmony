<script lang="ts">
	import { createInfiniteQuery, createMutation, useQueryClient } from '@tanstack/svelte-query'
	import { api } from '$lib/api.svelte'
	import { CheckIcon, XIcon } from '@lucide/svelte'
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte'
	import UserCard from '$lib/components/UserCard.svelte'

	const queryClient = useQueryClient()

	const incomingRequests = createInfiniteQuery(() => ({
		queryKey: ['friends', 'incoming'],
		initialPageParam: 0,
		queryFn({ pageParam }) {
			return api.getIncomingFriendRequests(pageParam)
		},
		getNextPageParam(lastPage) {
			return lastPage.has_more ? lastPage.offset + lastPage.limit : undefined
		},
		select(data) {
			return data.pages.flatMap((p) => p.data)
		}
	}))

	const outgoingRequests = createInfiniteQuery(() => ({
		queryKey: ['friends', 'outgoing'],
		initialPageParam: 0,
		queryFn({ pageParam }) {
			return api.getOutgoingFriendRequests(pageParam)
		},
		getNextPageParam(lastPage) {
			return lastPage.has_more ? lastPage.offset + lastPage.limit : undefined
		},
		select(data) {
			return data.pages.flatMap((p) => p.data)
		}
	}))

	const acceptRequest = createMutation(() => ({
		mutationKey: ['friends', 'accept'],
		mutationFn(userId: string) {
			return api.sendFriendRequestById(userId)
		},
		onSuccess() {
			queryClient.invalidateQueries({ queryKey: ['friends'] })
			queryClient.invalidateQueries({ queryKey: ['friends', 'incoming'] })
		}
	}))

	const rejectRequest = createMutation(() => ({
		mutationKey: ['friends', 'reject'],
		mutationFn(userId: string) {
			return api.rejectFriendRequest(userId)
		},
		onSuccess() {
			queryClient.invalidateQueries({ queryKey: ['friends', 'incoming'] })
		}
	}))

	const cancelRequest = createMutation(() => ({
		mutationKey: ['friends', 'cancel'],
		mutationFn(userId: string) {
			return api.removeFriend(userId)
		},
		onSuccess() {
			queryClient.invalidateQueries({ queryKey: ['friends', 'outgoing'] })
		}
	}))
</script>

<div class="flex h-full flex-col">
	<div class="border-b border-surface-300-700 px-6 py-4">
		<h2 class="text-xl font-semibold">Pending Requests</h2>
	</div>

	<div class="flex-1 overflow-y-auto p-4">
		<section class="mb-8">
			<h3 class="mb-3 text-sm font-semibold text-surface-600-400 uppercase">
				Incoming &ndash; {incomingRequests.data?.length ?? 0}
			</h3>

			{#if incomingRequests.isPending}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner class="size-8" />
				</div>
			{:else if incomingRequests.isError}
				<div class="rounded-container bg-surface-100-900 p-4 text-center">
					<p class="text-error-600-400">Failed to load requests</p>
				</div>
			{:else if incomingRequests.data.length === 0}
				<div class="rounded-container bg-surface-100-900 p-4 text-center">
					<p class="text-surface-600-400">No incoming requests</p>
				</div>
			{:else}
				<div class="grid gap-2">
					{#each incomingRequests.data as requester (requester.id)}
						<UserCard profile={requester} class="group w-full">
							<div class="ml-auto flex gap-1">
								<button
									onclick={() => acceptRequest.mutate(requester.id)}
									disabled={acceptRequest.isPending}
									class="btn-icon size-8 rounded-full opacity-0 transition-all group-hover:opacity-100 hover:preset-filled-success-500"
									title="Accept"
								>
									<CheckIcon class="size-4" />
								</button>
								<button
									onclick={() => rejectRequest.mutate(requester.id)}
									disabled={rejectRequest.isPending}
									class="btn-icon size-8 rounded-full opacity-0 transition-all group-hover:opacity-100 hover:preset-filled-error-500"
									title="Reject"
								>
									<XIcon class="size-4" />
								</button>
							</div>
						</UserCard>
					{/each}
				</div>
				{#if incomingRequests.hasNextPage}
					<button
						onclick={() => incomingRequests.fetchNextPage()}
						disabled={incomingRequests.isFetchingNextPage}
						class="mt-4 btn w-full preset-tonal"
					>
						{incomingRequests.isFetchingNextPage ? 'Loading...' : 'Load More'}
					</button>
				{/if}
			{/if}
		</section>
		<section>
			<h3 class="mb-3 text-sm font-semibold text-surface-600-400 uppercase">
				Outgoing &ndash; {outgoingRequests.data?.length ?? 0}
			</h3>

			{#if outgoingRequests.isPending}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner class="size-8" />
				</div>
			{:else if outgoingRequests.isError}
				<div class="rounded-container bg-surface-200-800 p-4 text-center">
					<p class="text-error-600-400">Failed to load requests</p>
				</div>
			{:else if outgoingRequests.data.length === 0}
				<div class="rounded-container bg-surface-200-800 p-4 text-center">
					<p class="text-surface-600-400">No outgoing requests</p>
				</div>
			{:else}
				<div class="grid gap-2">
					{#each outgoingRequests.data as requested (requested.id)}
						<UserCard profile={requested} class="group w-full">
							<div class="ml-auto flex">
								<button
									onclick={() => cancelRequest.mutate(requested.id)}
									disabled={cancelRequest.isPending}
									class="btn-icon size-8 rounded-full opacity-0 transition-all group-hover:opacity-100 hover:preset-filled-error-500"
									title="Cancel"
								>
									<XIcon class="size-4" />
								</button>
							</div>
						</UserCard>
					{/each}
				</div>
				{#if outgoingRequests.hasNextPage}
					<button
						onclick={() => outgoingRequests.fetchNextPage()}
						disabled={outgoingRequests.isFetchingNextPage}
						class="mt-4 btn w-full preset-tonal"
					>
						{outgoingRequests.isFetchingNextPage ? 'Loading...' : 'Load More'}
					</button>
				{/if}
			{/if}
		</section>
	</div>
</div>
