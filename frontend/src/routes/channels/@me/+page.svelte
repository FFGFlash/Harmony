<script lang="ts">
	import { createMutation, useQueryClient } from '@tanstack/svelte-query'
	import { api } from '$lib/api.svelte'
	import { Tabs } from '@skeletonlabs/skeleton-svelte'
	import FriendsList from '$lib/features/friends/FriendsList.svelte'
	import PendingRequests from '$lib/features/friends/PendingRequests.svelte'
	import AddFriend from '$lib/features/friends/AddFriend.svelte'

	let username = $state('')

	const queryClient = useQueryClient()

	const sendFriendRequest = createMutation(() => ({
		mutationKey: ['friends', 'outgoing'],
		mutationFn({ username }: { username: string }) {
			return api.sendFriendRequestByUsername(username)
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
</script>

<div class="flex">
	<div class="flex-1">
		<Tabs defaultValue="friends" class="h-dvh">
			<Tabs.List class="m-0 h-12 border-b border-surface-300-700 bg-surface-100-900 pb-0">
				<Tabs.Trigger value="friends">Friends</Tabs.Trigger>
				<Tabs.Trigger value="pending">Pending</Tabs.Trigger>
				<Tabs.Trigger value="add-friend">Add Friend</Tabs.Trigger>
				<Tabs.Indicator />
			</Tabs.List>

			<Tabs.Content value="friends" class="flex-1">
				<FriendsList />
			</Tabs.Content>

			<Tabs.Content value="pending" class="flex-1">
				<PendingRequests />
			</Tabs.Content>

			<Tabs.Content value="add-friend" class="flex-1">
				<AddFriend />
			</Tabs.Content>
		</Tabs>
	</div>
</div>
