<script lang="ts">
	import { goto } from '$app/navigation'
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte'
	import { auth } from '$lib/stores/auth.svelte'
	import { onMount } from 'svelte'

	onMount(async () => {
		await auth.initialize()

		if (!auth.isAuthenticated) {
			goto('/login')
			return
		}

		goto('/channels/@me')
	})
</script>

<div class="flex h-dvh items-center justify-center">
	<LoadingSpinner class="size-16" />
</div>
