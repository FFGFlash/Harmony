<script lang="ts">
	import { auth } from '$lib/stores/auth.svelte'
</script>

<div
	class="flex flex-1 flex-col items-center justify-center bg-linear-to-br from-primary-500 to-secondary-500"
>
	<div class="text-center">
		<h1 class="h1">Harmony</h1>
		<p class="mt-4 text-xl">Connect, Chat, and Collaborate</p>

		<div class="mt-8 flex items-center justify-center gap-4">
			{#if auth.isAuthenticated}
				<a href="/app" class="btn w-1/2 preset-filled-primary-500">Open App</a>
			{:else}
				<a href="/login" class="btn w-1/2 preset-filled-primary-500">Sign In</a>
				<a href="/register" class="btn w-1/2 preset-filled-primary-500">Sign Up</a>
			{/if}
		</div>
	</div>
</div>
