<script lang="ts">
	import { auth } from '$lib/stores/auth.svelte'
	import { RegisterRequestSchema } from '$lib/types'

	let email = $state('')
	let username = $state('')
	let password = $state('')
	let confirmPassword = $state('')
	let validationError = $state('')

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault()
		validationError = ''

		if (password !== confirmPassword) {
			validationError = 'Passwords do not match'
			return
		}

		const result = RegisterRequestSchema.safeParse({ username, email, password })

		if (!result.success) {
			validationError = result.error.issues[0].message
			return
		}

		try {
			await auth.register(result.data)
		} catch {}
	}
</script>

<div class="text-center">
	<h1 class="h1 text-4xl">Create an account</h1>
	<p class="mt-2 text-sm text-surface-700-300">Join Harmony today</p>
</div>

<form novalidate class="space-y-6" onsubmit={handleSubmit}>
	{#if validationError || auth.error}
		<span class="mb-2 block text-center text-error-600-400">{validationError || auth.error}</span>
	{/if}

	<fieldset class="flex flex-col items-center space-y-4">
		<label class="group label" for="email">
			<span class="label-text group-[&:has(:user-invalid)]:text-error-600-400">Email</span>
			<input
				class="input user-invalid:ring-error-600-400"
				name="email"
				id="email"
				type="email"
				autocomplete="off"
				bind:value={email}
				required
			/>
		</label>

		<label class="group label" for="username">
			<span class="label-text group-[&:has(:user-invalid)]:text-error-600-400"
				>Username (letters, numbers, _ and .)</span
			>
			<input
				class="input user-invalid:ring-error-600-400"
				name="username"
				id="username"
				type="username"
				autocomplete="off"
				pattern="^[a-zA-Z0-9._]+$"
				bind:value={username}
				required
			/>
		</label>

		<label class="group label" for="password">
			<span class="label-text group-[&:has(:user-invalid)]:text-error-600-400">Password</span>
			<input
				class="input user-invalid:ring-error-600-400"
				name="password"
				id="password"
				type="password"
				autocomplete="off"
				bind:value={password}
				required
				minlength="8"
			/>
		</label>

		<label class="group label" for="confirmPassword">
			<span class="label-text group-[&:has(:user-invalid)]:text-error-600-400"
				>Confirm Password</span
			>
			<input
				class="input user-invalid:ring-error-600-400"
				name="confirmPassword"
				id="confirmPassword"
				type="password"
				autocomplete="off"
				bind:value={confirmPassword}
				required
			/>
		</label>
	</fieldset>
	<button type="submit" disabled={auth.loading} class="btn w-full preset-filled-primary-500">
		{auth.loading ? 'Creating account...' : 'Create Account'}
	</button>
	<div class="text-center text-sm">
		<span class="text-surface-700-300">Already have an account?</span>
		<a class="ml-1 anchor" href="/login">Login</a>
	</div>
</form>
