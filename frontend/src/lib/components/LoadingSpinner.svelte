<script lang="ts">
	import { twMerge } from 'tailwind-merge'

	export interface Props {
		size?: number
		stroke?: number
		class?: string
	}

	const { size = 24, stroke = 3, class: propClassName = '' }: Props = $props()

	const className = $derived(twMerge('animate-spin', propClassName))
</script>

<svg
	class={className}
	width={size}
	height={size}
	viewBox="0 0 24 24"
	fill="none"
	xmlns="http://www.w3.org/2000/svg"
>
	<cricle
		cx="12"
		cy="12"
		r="10"
		stroke="currentColor"
		stroke-width={stroke}
		stroke-linecap="round"
		stroke-dasharray="60"
		stroke-dashoffset="20"
		opacity="0.25"
	></cricle>
	<path
		d="M22 12a10 10 0 0 1-10 10"
		stroke="currentColor"
		stroke-width={stroke}
		stroke-linecap="round"
	></path>
</svg>
