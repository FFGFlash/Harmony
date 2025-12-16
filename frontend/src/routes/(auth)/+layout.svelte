<script lang="ts">
	let { children } = $props()
</script>

<main
	class="flex h-dvh w-screen items-center justify-center bg-linear-to-br from-primary-500 to-secondary-500"
>
	<div class="w-full max-w-md space-y-8 rounded-container bg-surface-100-900 p-8 shadow-xl">
		{@render children()}
	</div>
</main>
