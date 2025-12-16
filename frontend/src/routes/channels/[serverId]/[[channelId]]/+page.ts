import { api } from '$lib/api.svelte'
import type { PageLoad } from './$types'

export const load: PageLoad = async ({ parent, params }) => {
	const { queryClient } = await parent()

	const server = await queryClient.ensureQueryData({
		queryKey: ['servers', params.serverId],
		queryFn: () => api.getServer(params.serverId)
	})

	const channels = await queryClient.ensureQueryData({
		queryKey: ['servers', params.serverId, 'channels'],
		queryFn: () => api.getServerChannels(params.serverId)
	})

	const channelId = params.channelId || server.main_channel_id || channels.at(0)?.id

	return { channelId }
}
