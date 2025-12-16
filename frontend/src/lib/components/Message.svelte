<script lang="ts">
	import { auth } from '$lib/stores/auth.svelte'
	import type { Message } from '$lib/types'

	interface Props {
		message: Message & { state?: 'pending' | 'failed' }
		onresend?: () => void
	}

	const { message, onresend }: Props = $props()
</script>

<div
	data-owned={auth.user?.id === message.user_id}
	data-state={message.state}
	class="flex justify-between"
>
	<span>{message.username}: {message.content}</span>
	{#if message.state === 'pending'}
		<span class="text-surface-400-600">pending...</span>
	{:else if message.state === 'failed'}
		<button onclick={onresend} class="anchor text-error-600-400">Resend</button>
	{/if}
</div>
