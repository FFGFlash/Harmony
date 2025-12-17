<script lang="ts">
	import { createInfiniteQuery, createMutation, useQueryClient } from '@tanstack/svelte-query'
	import { api } from '$lib/api.svelte'
	import UserCard from '$lib/components/UserCard.svelte'
	import { Tabs } from '@skeletonlabs/skeleton-svelte'
	import { CheckIcon, XIcon } from '@lucide/svelte'

	let username = $state('')

	const queryClient = useQueryClient()

	const sendFriendRequest = createMutation(() => ({
		mutationKey: ['friends', 'outgoing'],
		mutationFn({ username }: { username: string }) {
			return api.sendFriendRequest(username)
		},
		onSuccess() {
			queryClient.invalidateQueries({ queryKey: ['friends', 'outgoing'] })
		}
	}))

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault()
		const trimmedUsername = username.trim()
		if (trimmedUsername.length <= 0) return
		sendFriendRequest.mutate({ username: trimmedUsername })
	}

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
</script>

<div class="flex">
	<div class="flex-1">
		<Tabs defaultValue="friends">
			<Tabs.List>
				<Tabs.Trigger value="friends">Friends</Tabs.Trigger>
				<Tabs.Trigger value="pending">Pending</Tabs.Trigger>
				<Tabs.Trigger value="add-friend">Add Friend</Tabs.Trigger>
				<Tabs.Indicator />
			</Tabs.List>
			<Tabs.Content value="friends">
				Friends List
				<div class="flex flex-col">
					{#if friends.isPending}
						Loading...
					{:else if friends.isError}
						Error
					{:else}
						{#each friends.data as friend (friend.id)}
							<UserCard profile={friend} />
						{/each}
					{/if}
				</div>
			</Tabs.Content>
			<Tabs.Content value="pending" class="flex flex-col gap-4">
				<div class="flex flex-col gap-2 px-2">
					<span>Incoming Requests</span>
					{#if incomingRequests.isPending}
						Loading...
					{:else if incomingRequests.isError}
						Error
					{:else}
						{#each incomingRequests.data as request (request.id)}
							<UserCard profile={request} class="group">
								<div class="ml-auto flex">
									<button
										class="btn-icon rounded-full bg-transparent opacity-0 transition-all group-hover:opacity-100 hover:preset-filled-error-500"
									>
										<XIcon class="size-8" />
									</button>
									<button
										class="btn-icon rounded-full bg-transparent opacity-0 transition-all group-hover:opacity-100 hover:preset-filled-success-500"
									>
										<CheckIcon class="size-8" />
									</button>
								</div>
							</UserCard>
						{/each}
					{/if}
				</div>
				<div class="flex flex-col gap-2 px-2">
					<span>Outgoing Requests</span>
					{#if outgoingRequests.isPending}
						Loading...
					{:else if outgoingRequests.isError}
						Error
					{:else}
						{#each outgoingRequests.data as request (request.id)}
							<UserCard profile={request} class="group">
								<div class="ml-auto flex">
									<button
										class="btn-icon rounded-full bg-transparent opacity-0 transition-all group-hover:opacity-100 hover:preset-filled-error-500"
									>
										<XIcon class="size-8" />
									</button>
								</div>
							</UserCard>
						{/each}
					{/if}
				</div>
			</Tabs.Content>
			<Tabs.Content value="add-friend">
				<form onsubmit={handleSubmit}>
					<input type="text" placeholder="Enter a username" bind:value={username} class="input" />
				</form>
			</Tabs.Content>
		</Tabs>
	</div>
</div>
