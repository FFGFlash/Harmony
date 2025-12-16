import { UserSchema, type User, type LoginRequest, type RegisterRequest } from '$lib/types'
import debug from 'debug'
import { websocket } from './websocket.svelte'
import { goto } from '$app/navigation'
import { resolve } from '$app/paths'
import ensureError from '$lib/ensureError'
import { api } from '$lib/api.svelte'

const log = debug('harmony:auth-store')

class AuthStore {
	user = $state<User | null>(null)
	token = $state<string | null>(null)
	loading = $state(false)
	error = $state<string | null>(null)

	get isAuthenticated() {
		return !!this.user && !!this.token
	}

	async initialize() {
		const storedToken = localStorage.getItem('token')
		const storedUser = localStorage.getItem('user')

		if (storedToken && storedUser) {
			try {
				this.token = storedToken
				this.user = UserSchema.parse(JSON.parse(storedUser))
				websocket.connect(storedToken)
			} catch (error) {
				log('Failed to restore auth state', error)
				this.logout()
			}
		}
	}

	async login(data: LoginRequest) {
		this.loading = true
		this.error = null

		try {
			const response = await api.login(data)
			this.setAuth(response.user, response.token)
			await goto(resolve('/app'))
		} catch (error) {
			this.error = ensureError(error).message || 'Login failed'
			throw error
		} finally {
			this.loading = false
		}
	}

	async register(data: RegisterRequest) {
		this.loading = true
		this.error = null

		try {
			const response = await api.register(data)
			this.setAuth(response.user, response.token)
			await goto(resolve('/app'))
		} catch (error) {
			this.error = ensureError(error).message || 'Registration failed'
			throw error
		} finally {
			this.loading = false
		}
	}

	async logout() {
		this.user = null
		this.token = null
		this.error = null

		localStorage.removeItem('token')
		localStorage.removeItem('user')

		websocket.disconnect()
		await goto(resolve('/login'))
	}

	private setAuth(user: User, token: string) {
		this.user = user
		this.token = token

		localStorage.setItem('token', token)
		localStorage.setItem('user', JSON.stringify(user))

		websocket.connect(token)
	}
}

export const auth = new AuthStore()
