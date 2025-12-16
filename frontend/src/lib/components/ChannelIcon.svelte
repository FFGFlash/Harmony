<script lang="ts">
	import type { ChannelType } from '$lib/types'
	import { AudioLinesIcon, HashIcon } from '@lucide/svelte'

	export interface Props {
		type: ChannelType
		class?: string
	}

	const { type, class: className }: Props = $props()
</script>

{#if type === 'text'}
	<HashIcon class={className} />
{:else if type === 'voice'}
	<AudioLinesIcon class={className} />
{/if}
