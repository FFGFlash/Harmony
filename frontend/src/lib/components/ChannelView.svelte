<script lang="ts">
	import type { Channel, Message as TMessage } from '$lib/types'
	import {
		createInfiniteQuery,
		createMutation,
		useQueryClient,
		type InfiniteData
	} from '@tanstack/svelte-query'
	import ChannelIcon from './ChannelIcon.svelte'
	import { api } from '$lib/api.svelte'
	import { auth } from '$lib/stores/auth.svelte'
	import Message from './Message.svelte'
	import { websocket } from '$lib/stores/websocket.svelte'

	interface MessageWithState extends TMessage {
		state?: 'pending' | 'failed'
	}

	export interface Props {
		channel: Channel
	}

	const { channel }: Props = $props()

	type EventWithTarget<E, T> = E & { currentTarget: EventTarget & T }

	const queryClient = useQueryClient()

	const messageHistory = createInfiniteQuery(() => ({
		queryKey: ['channels', channel.id, 'messages'],
		initialPageParam: undefined as string | undefined,
		queryFn({ pageParam }): Promise<MessageWithState[]> {
			return api.getMessages(channel.id, undefined, pageParam)
		},
		getNextPageParam(lastPage) {
			return lastPage.at(-1)?.id
		},
		select(data) {
			return data.pages.flat()
		}
	}))

	type SendMessageVariables = { content: string; tempId: string }

	const failedMessages = $state(new Map<string, SendMessageVariables>())

	const sendMessageMutation = createMutation(() => ({
		async mutationFn({ content, tempId }: SendMessageVariables) {
			const result = await api.sendMessage(channel.id, content)
			return { message: result, tempId }
		},
		async onMutate({ content, tempId }) {
			await queryClient.cancelQueries({ queryKey: ['channels', channel.id, 'messages'] })

			const previousMessages = queryClient.getQueryData(['channels', channel.id, 'messages'])

			const optimisticMessage: MessageWithState = {
				id: tempId,
				channel_id: channel.id,
				user_id: auth.user!.id,
				username: auth.user!.username,
				content,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString(),
				state: 'pending'
			}

			queryClient.setQueryData(
				['channels', channel.id, 'messages'],
				(old: InfiniteData<MessageWithState[], string | undefined>) => {
					if (!old) return old
					return {
						...old,
						pages: old.pages.with(0, [optimisticMessage, ...(old.pages.at(0) ?? [])])
					}
				}
			)

			return { previousMessages, optimisticMessage }
		},
		onSuccess(data, variables) {
			queryClient.setQueryData(
				['channels', channel.id, 'messages'],
				(old: InfiniteData<MessageWithState[], string | undefined>) => {
					if (!old) return old
					const filtered = old.pages.at(0)?.filter((m) => m.id !== data.tempId) ?? []

					const hasRealMessage = filtered?.some((m) => m.id === data.message.id)

					if (!hasRealMessage) {
						return {
							...old,
							pages: old.pages.with(0, [data.message, ...filtered])
						}
					}

					return {
						...old,
						pages: old.pages.with(0, filtered)
					}
				}
			)

			failedMessages.delete(variables.tempId)
		},
		onError(_err, variables) {
			queryClient.setQueryData(
				['channels', channel.id, 'messages'],
				(old: InfiniteData<MessageWithState[], string | undefined>) => {
					if (!old) return old
					return {
						...old,
						pages: old.pages.map((p) => {
							return p.map((m) => (m.id === variables.tempId ? { ...m, state: 'failed' } : m))
						})
					}
				}
			)

			failedMessages.set(variables.tempId, {
				content: variables.content,
				tempId: variables.tempId
			})
		}
	}))

	let content = $state('')

	function handleSubmit(e: EventWithTarget<SubmitEvent, HTMLFormElement>) {
		e.preventDefault()

		if (!content.trim()) return

		const messageContent = content.trim()
		const tempId = crypto.randomUUID()
		content = ''

		sendMessageMutation.mutate({ content: messageContent, tempId })
	}

	function retryMessage(messageId: string) {
		const failedMessage = failedMessages.get(messageId)
		if (!failedMessage) return

		const newTempId = crypto.randomUUID()

		queryClient.setQueryData(
			['channels', channel.id, 'messages'],
			(old: InfiniteData<MessageWithState[], string | undefined>) => {
				if (!old) return old
				return {
					...old,
					pages: old.pages.map((p) => p.filter((m) => m.id !== messageId))
				}
			}
		)

		failedMessages.delete(messageId)

		sendMessageMutation.mutate({ content: failedMessage.content, tempId: newTempId })
	}

	function handleWebSocketMessage(message: TMessage) {
		if (message.channel_id !== channel.id) return

		queryClient.setQueryData(
			['channels', channel.id, 'messages'],
			(old: InfiniteData<MessageWithState[], string | undefined>) => {
				if (!old) return old
				const exists = old.pages[0].some((m) => m.id === message.id)
				if (exists) return old
				return {
					...old,
					pages: old.pages.with(0, [message, ...old.pages[0]])
				}
			}
		)
	}

	$effect(() => {
		websocket.subscribe(channel.id)

		const unsubscribe = websocket.onMessage((wsMessage) => {
			if (wsMessage.type === 'message_created') {
				handleWebSocketMessage({
					id: wsMessage.id,
					channel_id: wsMessage.channel_id,
					user_id: wsMessage.user_id,
					username: wsMessage.username,
					content: wsMessage.content,
					created_at: wsMessage.created_at,
					updated_at: wsMessage.updated_at
				})
			}
		})

		return () => {
			websocket.unsubscribe(channel.id)
			unsubscribe()
		}
	})
</script>

<div class="flex h-full flex-1 flex-col">
	<div class="flex h-12 items-center gap-1 border-b border-surface-300-700 px-4 text-base">
		<ChannelIcon type={channel.channel_type} class="size-4" />
		<span>{channel.name}</span>
	</div>
	<div class="flex flex-1 flex-col-reverse overflow-y-auto px-4">
		{#if messageHistory.isPending}
			<div>Loading...</div>
		{:else if messageHistory.isError}
			<div>Error: {messageHistory.error.message}</div>
		{:else}
			{#each messageHistory.data as message}
				<Message {message} />
			{/each}
		{/if}
	</div>
	<div class="h-18 px-4 py-2">
		<form
			class="flex size-full rounded-md border border-surface-400-600 bg-surface-300-700 p-2 shadow-md"
			onsubmit={handleSubmit}
		>
			<input
				type="text"
				class="flex-1 input-ghost"
				placeholder={`Message #${channel.name}`}
				bind:value={content}
			/>
		</form>
	</div>
</div>
