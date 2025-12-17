import { env } from '$env/dynamic/public'
import { z, type ZodType } from 'zod'
import {
	AuthResponseSchema,
	ChannelSchema,
	ErrorResponseSchema,
	FriendshipSchema,
	FullProfileSchema,
	MessageSchema,
	PaginatedResponse,
	ProfileSchema,
	ServerSchema,
	type ErrorResponse,
	type LoginRequest,
	type RegisterRequest
} from './types'
import { browser, dev } from '$app/environment'

export const API_URL = env.PUBLIC_API_URL || ''

export class ApiError extends Error {
	constructor(
		public status: number,
		message: string
	) {
		super(message)
		this.name = 'ApiError'
	}
}

class Api {
	private async fetch<T>(
		endpoint: string,
		options: RequestInit = {},
		schema?: ZodType<T>
	): Promise<T> {
		const token = localStorage.getItem('token')

		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			...options.headers,
			...(token && { Authorization: `Bearer ${token}` })
		}

		const response = await fetch(`${API_URL}${endpoint}`, { ...options, headers })

		if (!response.ok) {
			const error = await response
				.json()
				.then((j) => ErrorResponseSchema.parse(j))
				.catch<ErrorResponse>(() => ({ message: 'An error occurred' }))
			throw new ApiError(response.status, error.message || error.error || 'An error occurred')
		}

		const json = await response.json()

		if (!schema) return json

		const parsed = schema.safeParse(json)

		if (parsed.success) return parsed.data

		throw new ApiError(response.status, 'Unexpected response')
	}

	async register(data: RegisterRequest) {
		return this.fetch(
			'/api/auth/register',
			{ method: 'POST', body: JSON.stringify(data) },
			AuthResponseSchema
		)
	}

	async login(data: LoginRequest) {
		return this.fetch(
			'/api/auth/login',
			{ method: 'POST', body: JSON.stringify(data) },
			AuthResponseSchema
		)
	}

	async getServers() {
		return this.fetch('/api/servers', undefined, z.array(ServerSchema))
	}

	async createServer(name: string) {
		return this.fetch(
			`/api/servers`,
			{ method: 'POST', body: JSON.stringify({ name }) },
			ServerSchema
		)
	}

	async getServer(serverId: string) {
		return this.fetch(`/api/servers/${serverId}`, undefined, ServerSchema)
	}

	async deleteServer(serverId: string) {
		return this.fetch<void>(`/api/servers/${serverId}`, { method: 'DELETE' })
	}

	async getServerChannels(serverId: string) {
		return this.fetch(`/api/servers/${serverId}/channels`, undefined, z.array(ChannelSchema))
	}

	async createChannel(serverId: string, name: string) {
		return this.fetch(
			`/api/servers/${serverId}/channels`,
			{
				method: 'POST',
				body: JSON.stringify({ name })
			},
			ChannelSchema
		)
	}

	async deleteChannel(channelId: string) {
		return this.fetch<void>(`/api/channels/${channelId}`, { method: 'DELETE' })
	}

	async getMessages(channelId: string, limit = 50, before?: string) {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams({ limit: limit.toString() })
		if (before) params.append('before', before)
		return this.fetch(
			`/api/channels/${channelId}/messages?${params}`,
			undefined,
			z.array(MessageSchema)
		)
	}

	async sendMessage(channelId: string, content: string) {
		return this.fetch(
			`/api/channels/${channelId}/messages`,
			{ method: 'POST', body: JSON.stringify({ content }) },
			MessageSchema
		)
	}

	async getDMs() {
		return this.fetch('/api/dms', undefined, z.array(ChannelSchema))
	}

	async createDM(recipientId: string) {
		return this.fetch(
			'/api/dms',
			{ method: 'POST', body: JSON.stringify({ recipient_id: recipientId }) },
			ChannelSchema
		)
	}

	async getUserByUsername(username: string) {
		return this.fetch(`/api/users/username/${username}`, undefined, ProfileSchema)
	}

	async sendFriendRequestByUsername(username: string) {
		return this.fetch(
			`/api/friends`,
			{ method: 'POST', body: JSON.stringify({ username }) },
			FriendshipSchema
		)
	}

	async sendFriendRequestById(id: string) {
		return this.fetch(`/api/users/${id}/friend`, { method: 'POST' }, FriendshipSchema)
	}

	async rejectFriendRequest(id: string): Promise<void> {
		return this.fetch(`/api/users/${id}/friend/reject`, { method: 'POST' })
	}

	async removeFriend(id: string): Promise<void> {
		return this.fetch(`/api/users/${id}/friend`, { method: 'DELETE' })
	}

	async getFriends(offset?: number) {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams()
		if (offset != null) params.append('offset', offset.toString())
		return this.fetch(`/api/friends?${params}`, undefined, PaginatedResponse(FullProfileSchema))
	}

	async getIncomingFriendRequests(offset?: number) {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams()
		if (offset != null) params.append('offset', offset.toString())
		return this.fetch(
			`/api/friends/incoming?${params}`,
			undefined,
			PaginatedResponse(FullProfileSchema)
		)
	}

	async getOutgoingFriendRequests(offset?: number) {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams()
		if (offset != null) params.append('offset', offset.toString())
		return this.fetch(
			`/api/friends/outgoing?${params}`,
			undefined,
			PaginatedResponse(FullProfileSchema)
		)
	}

	async getServerMembers(serverId: string, offset?: number) {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams()
		if (offset != null) params.append('offset', offset.toString())
		return this.fetch(
			`/api/servers/${serverId}/members?${params}`,
			undefined,
			PaginatedResponse(FullProfileSchema)
		)
	}

	async searchUsers(username: string) {
		const params = new URLSearchParams({ username })
		return this.fetch(
			`/api/users/search?${params}`,
			undefined,
			PaginatedResponse(FullProfileSchema)
		)
	}
}

export const api = new Api()

if (browser && dev) {
	//@ts-expect-error debugging
	globalThis.api = api
}
