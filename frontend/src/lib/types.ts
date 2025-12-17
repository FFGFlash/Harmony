import z from 'zod'

export const ErrorResponseSchema = z.object({
	error: z.string().optional(),
	message: z.string().optional()
})

export type ErrorResponse = z.infer<typeof ErrorResponseSchema>

export const UserSchema = z.object({
	id: z.uuid(),
	username: z.string(),
	email: z.email(),
	created_at: z.iso.datetime()
})

export type User = z.infer<typeof UserSchema>

export const LoginRequestSchema = z
	.object({
		email: z.email().optional(),
		username: z.string().optional(),
		password: z.string().min(8, 'Password must be at least 8 characters')
	})
	.refine((data) => data.email || data.username, {
		error: 'Either email or username must be provided'
	})

export const RegisterRequestSchema = z.object({
	username: z
		.string()
		.min(3, 'Username must be at least 3 characters')
		.max(32, 'Username must be at most 32 characters'),
	email: z.email(),
	password: z.string().min(8, 'Password must be at least 8 characters')
})

export type LoginRequest = z.infer<typeof LoginRequestSchema>
export type RegisterRequest = z.infer<typeof RegisterRequestSchema>

export const AuthResponseSchema = z.object({
	user: UserSchema,
	token: z.string()
})

export type AuthResponse = z.infer<typeof AuthResponseSchema>

export const ServerSchema = z.object({
	id: z.uuid(),
	name: z.string(),
	owner_id: z.uuid(),
	main_channel_id: z.uuid().optional().nullable(),
	is_owner: z.boolean(),
	created_at: z.iso.datetime()
})

export type Server = z.infer<typeof ServerSchema>

export const ChannelTypeSchema = z.enum(['text', 'voice', 'dm', 'group_dm'])
export type ChannelType = z.infer<typeof ChannelTypeSchema>

export const ChannelSchema = z.object({
	id: z.uuid(),
	server_id: z.uuid().optional().nullable(),
	name: z.string(),
	position: z.number(),
	channel_type: ChannelTypeSchema,
	topic: z.string().optional().nullable(),
	is_private: z.boolean(),
	created_at: z.iso.datetime()
})

export type Channel = z.infer<typeof ChannelSchema>

export const MessageSchema = z.object({
	id: z.uuid(),
	channel_id: z.uuid(),
	user_id: z.uuid(),
	username: z.string(),
	content: z.string(),
	created_at: z.iso.datetime(),
	updated_at: z.iso.datetime()
})

export type Message = z.infer<typeof MessageSchema>

export const WsMessageSchema = z.discriminatedUnion('type', [
	z.object({
		type: z.literal('subscribe'),
		channel_id: z.uuid()
	}),
	z.object({
		type: z.literal('unsubscribe'),
		channel_id: z.uuid()
	}),
	z.object({
		type: z.literal('message_created'),
		id: z.uuid(),
		channel_id: z.uuid(),
		user_id: z.uuid(),
		username: z.string(),
		content: z.string(),
		created_at: z.iso.datetime(),
		updated_at: z.iso.datetime()
	}),
	z.object({
		type: z.literal('subscribed'),
		channel_id: z.uuid()
	}),
	z.object({
		type: z.literal('unsubscribed'),
		channel_id: z.uuid()
	}),
	z.object({
		type: z.literal('error'),
		message: z.string()
	})
])

export type WsMessage = z.infer<typeof WsMessageSchema>

export const ProfileSchema = z.object({
	user_id: z.uuid(),
	display_name: z.string().optional().nullable(),
	bio: z.string().optional().nullable(),
	avatar_url: z.string().optional().nullable(),
	banner_url: z.string().optional().nullable(),
	status: z.enum(['online', 'away', 'dnd', 'offline']),
	custom_status: z.string().optional().nullable(),
	status_emoji: z.string().optional().nullable(),
	show_online_status: z.boolean(),
	allow_dms: z.boolean(),
	created_at: z.iso.datetime(),
	updated_at: z.iso.datetime()
})

export type Profile = z.infer<typeof ProfileSchema>

export const FullProfileSchema = z.object({
	id: z.uuid(),
	username: z.string(),
	display_name: z.string().optional().nullable(),
	bio: z.string().optional().nullable(),
	avatar_url: z.string().optional().nullable(),
	banner_url: z.string().optional().nullable(),
	status: z.enum(['online', 'away', 'dnd', 'offline']),
	custom_status: z.string().optional().nullable(),
	status_emoji: z.string().optional().nullable(),
	show_online_status: z.boolean(),
	created_at: z.iso.datetime()
})

export type FullProfile = z.infer<typeof FullProfileSchema>

export const FriendshipSchema = z.object({
	user_low: z.uuid(),
	user_high: z.uuid(),
	sender_id: z.uuid(),
	status: z.enum(['pending', 'accpted', 'rejected', 'blocked']),
	created_at: z.iso.datetime(),
	updated_at: z.iso.datetime()
})

export type Friendship = z.infer<typeof FriendshipSchema>

export function PaginatedResponse<T extends z.ZodType>(schema: T) {
	return z.object({
		data: z.array(schema),
		limit: z.number(),
		offset: z.number(),
		has_more: z.boolean()
	})
}
