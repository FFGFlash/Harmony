import { browser } from '$app/environment'
import debug from 'debug'
import z from 'zod'

const STORAGE_KEY = 'harmony_channel_history'
const MAX_HISTORY_AGE = 30 * 24 * 60 * 60 * 1000 // 30 days

const ChannelHistoryEntrySchema = z.object({ channelId: z.string(), lastVisited: z.number() })
const ChannelHistorySchema = z.record(z.string(), ChannelHistoryEntrySchema)

type ChannelHistory = z.infer<typeof ChannelHistorySchema>

const log = debug('harmony:channel-history')

class ChannelHistoryStore {
	#history = $state<ChannelHistory>({})

	constructor() {
		if (!browser) return
		this.loadFromStorage()
		this.cleanOldEntries()
	}

	private loadFromStorage() {
		try {
			const stored = localStorage.getItem(STORAGE_KEY)
			if (stored) this.#history = JSON.parse(stored)
		} catch (error) {
			log('Failed to load channel history', error)
			this.#history = {}
		}
	}

	private saveToStorage() {
		if (!browser) return

		try {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(this.#history))
		} catch (error) {
			log('Failed to save channel history', error)
		}
	}

	private cleanOldEntries() {
		const now = Date.now()
		let hasChanges = false

		for (const [serverId, entry] of Object.entries(this.#history)) {
			if (now - entry.lastVisited > MAX_HISTORY_AGE) {
				delete this.#history[serverId]
				hasChanges = true
			}
		}

		if (hasChanges) this.saveToStorage()
	}

	getLastChannel(serverId: string, mainChannelId?: string | null) {
		const entry = this.#history[serverId]

		if (!entry) return mainChannelId ?? ''

		if (Date.now() - entry.lastVisited > MAX_HISTORY_AGE) {
			delete this.#history[serverId]
			this.saveToStorage()
			return mainChannelId ?? ''
		}

		return entry.channelId
	}

	setLastChannel(serverId: string, channelId?: string | null) {
		log('Setting channel history for %s to %s', serverId, channelId)
		if (!channelId) delete this.#history[serverId]
		else this.#history[serverId] = { channelId, lastVisited: Date.now() }
		this.saveToStorage()
	}

	hasHistory(serverId: string) {
		return serverId in this.#history
	}

	getAllHistory() {
		return { ...this.#history }
	}

	clearHistory(serverId?: string) {
		if (serverId == null) this.#history = {}
		else delete this.#history[serverId]
		this.saveToStorage()
	}

	exportHistory() {
		return JSON.stringify(this.#history, null, 2)
	}

	importHistory(jsonString: string) {
		try {
			const imported = JSON.parse(jsonString)
			const parsed = ChannelHistorySchema.parse(imported)
			this.#history = parsed
			this.saveToStorage()
			return true
		} catch (error) {
			log('Failed to import history', error)
			return false
		}
	}
}

export const channelHistory = new ChannelHistoryStore()
