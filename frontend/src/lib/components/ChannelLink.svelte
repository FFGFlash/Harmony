<script lang="ts">
	import type { Channel } from '$lib/types'

	export interface Props {
		channel: Channel
	}

	const { channel }: Props = $props()

	const href = $derived(
		channel.server_id != null
			? `/channels/${channel.server_id}/${channel.id}`
			: `/channels/@me/${channel.id}`
	)
</script>

<a {href}>
	{channel.name}
</a>
