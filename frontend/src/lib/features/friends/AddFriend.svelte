<script lang="ts">
	import { api } from '$lib/api.svelte'
	import { CheckCircle2Icon, UserPlusIcon, XCircleIcon } from '@lucide/svelte'
	import { createMutation, useQueryClient } from '@tanstack/svelte-query'

	let username = $state('')
	let lastSubmittedUsername = $state('')

	const queryClient = useQueryClient()

	const sendFriendRequest = createMutation(() => ({
		mutationKey: ['friends', 'request'],
		mutationFn(username: string) {
			return api.sendFriendRequestByUsername(username)
		},
		onSuccess(_, username) {
			lastSubmittedUsername = username
			username = ''
			queryClient.invalidateQueries({ queryKey: ['friends', 'outgoing'] })
		},
		onError() {
			lastSubmittedUsername = username
		}
	}))

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault()
		const trimmed = username.trim()
		if (trimmed.length <= 0) return
		sendFriendRequest.mutate(trimmed)
	}

	$effect(() => {
		if (username !== lastSubmittedUsername) {
			sendFriendRequest.reset()
		}
	})
</script>

<div class="flex h-full flex-col">
	<div class="border-b border-surface-300-700 px-6 py-4">
		<h2 class="text-xl font-semibold">Add Friend</h2>
		<p class="mt-1 text-sm text-surface-600-400">You can add friends by entering their username.</p>
	</div>

	<div class="flex-1 overflow-y-auto p-6">
		<form onsubmit={handleSubmit} class="mx-auto max-w-2xl">
			<div class="rounded-container bg-surface-100-900 p-6">
				<label class="label mb-4">
					<span class="label-text text-sm font-semibold uppercase">Username</span>
					<div class="relative">
						<input
							type="text"
							placeholder="Enter a username"
							bind:value={username}
							class="input pr-24"
							disabled={sendFriendRequest.isPending}
						/>
						<button
							type="submit"
							disabled={sendFriendRequest.isPending || !username.trim()}
							class="absolute top-1/2 right-2 btn -translate-y-1/2 preset-filled-primary-500"
						>
							{#if sendFriendRequest.isPending}
								Sending...
							{:else}
								Send Request
							{/if}
						</button>
					</div>
				</label>

				{#if sendFriendRequest.isSuccess}
					<div
						class="mt-4 flex items-center gap-2 rounded-container bg-success-500/10 p-4 text-success-600-400"
					>
						<CheckCircle2Icon class="size-5 shrink-0" />
						<div>
							<p class="font-semibold">Success!</p>
							<p class="text-sm">
								Friend request sent to <span class="font-semibold">{lastSubmittedUsername}</span>
							</p>
						</div>
					</div>
				{:else if sendFriendRequest.isError}
					<div
						class="mt-4 flex items-center gap-2 rounded-container bg-error-500/10 p-4 text-error-600-400"
					>
						<XCircleIcon class="size-5 shrink-0" />
						<div>
							<p class="font-semibold">Failed to send request</p>
							<p class="text-sm">
								{sendFriendRequest.error?.message || 'Something went wrong'}
							</p>
						</div>
					</div>
				{/if}
			</div>

			<div class="mt-6 rounded-container bg-surface-100-900 p-6">
				<div class="flex items-start gap-3">
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary-500/20"
					>
						<UserPlusIcon class="size-5 text-primary-500" />
					</div>
					<div>
						<h3 class="font-semibold">How to add friends</h3>
						<p class="mt-1 text-sm text-surface-600-400">
							Enter the exact username of the person you want to add. Usernames are case-sensitive
							and must match exactly.
						</p>
					</div>
				</div>
			</div>
		</form>
	</div>
</div>
