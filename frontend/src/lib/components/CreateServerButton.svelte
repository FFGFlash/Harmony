<script lang="ts">
	import { api } from '$lib/api.svelte'
	import { Dialog, Portal } from '@skeletonlabs/skeleton-svelte'
	import { XIcon, Plus } from '@lucide/svelte'
	import type { Server } from '$lib/types'

	const { onCreateServer }: { onCreateServer?: (server: Server) => any } = $props()

	let open = $state(false)
	let name = $state('')
	let validationError = $state('')

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault()

		if (!name) {
			validationError = 'Name is required'
			return
		}

		const server = await api.createServer(name)

		await onCreateServer?.(server)

		open = false
	}

	function handleOpen(details: { open: boolean }) {
		open = details.open
		if (details.open) name = ''
	}
</script>

<Dialog {open} onOpenChange={handleOpen}>
	<Dialog.Trigger
		class="btn-icon size-12 rounded-2xl preset-filled-surface-300-700 p-0 transition-all hover:rounded-xl"
		title="Add a Server"
	>
		<Plus class="size-6" />
	</Dialog.Trigger>
	<Portal>
		<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50 backdrop-blur-xs" />
		<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
			<Dialog.Content
				class="w-full max-w-md translate-y-[10px] space-y-4 card bg-surface-100-900 p-4 opacity-0 shadow-xl transition transition-discrete data-[state=open]:translate-y-0 data-[state=open]:opacity-100 starting:data-[state=open]:translate-y-[10px] starting:data-[state=open]:opacity-0"
			>
				<form class="contents space-y-4" onsubmit={handleSubmit}>
					<header class="flex items-center justify-between">
						<Dialog.Title class="text-lg font-bold">Create Server</Dialog.Title>
						<Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
							<XIcon class="size-4" />
						</Dialog.CloseTrigger>
					</header>
					<fieldset class="space-y-4">
						<label class="label" for="name">
							<span class="label-text">Name</span>
							<input type="text" class="input" name="name" id="name" bind:value={name} required />
						</label>
					</fieldset>
					<footer class="flex justify-end gap-2">
						<Dialog.CloseTrigger class="btn preset-tonal">Cancel</Dialog.CloseTrigger>
						<button class="btn preset-filled">Create</button>
					</footer>
				</form>
			</Dialog.Content>
		</Dialog.Positioner>
	</Portal>
</Dialog>
