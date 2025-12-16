import { env } from '$env/dynamic/public'
import { WsMessageSchema, type WsMessage } from '$lib/types'
import debug from 'debug'

const WS_URL = env.PUBLIC_WS_URL || `ws://${location.host}`

export type MessageHandler = (message: WsMessage) => void

const log = debug('harmony:websocket-store')

class WebSocketStore {
	private ws = $state<WebSocket | null>(null)
	private reconnectTimeout: ReturnType<typeof setTimeout> | null = null
	private messageHandlers = new Set<MessageHandler>()

	connected = $state(false)
	subscribedChannels = $state(new Set<string>())

	connect(token: string) {
		if (this.ws?.readyState === WebSocket.OPEN) return

		try {
			this.ws = new WebSocket(`${WS_URL}/ws?token=${token}`)

			this.ws.onopen = () => {
				log('Connected')
				this.connected = true

				this.subscribedChannels.forEach((channel_id) => {
					this.send({ type: 'subscribe', channel_id })
				})
			}

			this.ws.onmessage = (event) => {
				try {
					const data = JSON.parse(event.data)
					const message = WsMessageSchema.parse(data)
					this.messageHandlers.forEach((handler) => handler(message))
				} catch (error) {
					log('Failed to parse message', error)
				}
			}

			this.ws.onclose = () => {
				log('Disconnected')
				this.connected = false
				this.scheduleReconnect(token)
			}

			this.ws.onerror = (error) => {
				log('error', error)
			}
		} catch (error) {
			log('Failed to connect', error)
			this.scheduleReconnect(token)
		}
	}

	disconnect() {
		if (this.reconnectTimeout) {
			clearTimeout(this.reconnectTimeout)
			this.reconnectTimeout = null
		}

		if (this.ws) {
			this.ws.close()
			this.ws = null
		}

		this.connected = false
		this.subscribedChannels.clear()
	}

	send(message: WsMessage) {
		if (this.ws?.readyState !== WebSocket.OPEN) return
		this.ws.send(JSON.stringify(message))
	}

	private scheduleReconnect(token: string) {
		if (this.reconnectTimeout) clearTimeout(this.reconnectTimeout)

		this.reconnectTimeout = setTimeout(() => {
			log('Attempting to reconnect...')
			this.connect(token)
		}, 3000)
	}

	subscribe(channelId: string) {
		log('subscribed to %s', channelId)
		this.subscribedChannels.add(channelId)
		if (this.connected) this.send({ type: 'subscribe', channel_id: channelId })
	}

	unsubscribe(channelId: string) {
		log('unsubscribed to %s', channelId)
		this.subscribedChannels.delete(channelId)
		if (this.connected) this.send({ type: 'unsubscribe', channel_id: channelId })
	}

	onMessage(handler: MessageHandler) {
		this.messageHandlers.add(handler)
		return () => this.messageHandlers.delete(handler)
	}
}

export const websocket = new WebSocketStore()
