<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton-svelte'

	let { children } = $props()
</script>

<div class="flex min-h-dvh flex-col">
	<!-- <AppBar>
		<AppBar.Toolbar class="grid-cols-[1fr_auto]">
			<AppBar.Headline>
				<p class="text-3xl">Harmony</p>
			</AppBar.Headline>
			<AppBar.Trail>
				<a href="/login" class="btn preset-filled-primary-500">Login</a>
				<a href="/register" class="btn preset-filled-primary-500">Register</a>
			</AppBar.Trail>
		</AppBar.Toolbar>
	</AppBar> -->

	{@render children()}
</div>
