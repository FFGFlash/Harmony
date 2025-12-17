<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query'
	import { api } from '$lib/api.svelte'
	import {
		Combobox,
		Portal,
		useListCollection,
		type ComboboxRootProps
	} from '@skeletonlabs/skeleton-svelte'
	import UserCard from './UserCard.svelte'

	let query = $state('')
	let debouncedQuery = $state('')

	$effect(() => {
		query

		const handler = setTimeout(() => {
			debouncedQuery = query
		}, 500)

		return () => clearTimeout(handler)
	})

	const result = createQuery(() => ({
		queryKey: ['users', 'search', debouncedQuery],
		enabled: debouncedQuery.length > 0,
		queryFn() {
			return api.searchUsers(debouncedQuery)
		},
		select(data) {
			return data.data
		}
	}))

	const collection = $derived(
		useListCollection({
			items: result.data ?? [],
			itemToString: (item) => item.username,
			itemToValue: (item) => item.id
		})
	)

	const onInputValueChange: ComboboxRootProps['onInputValueChange'] = (e) => {
		query = e.inputValue
	}

	const onSelect: ComboboxRootProps['onSelect'] = (e) => {
		console.log(e)
		query = ''
	}
</script>

<Combobox
	placeholder="Search..."
	{collection}
	{onInputValueChange}
	inputValue={query}
	value={[]}
	{onSelect}
>
	<Combobox.Control>
		<Combobox.Input />
		<Combobox.Trigger />
	</Combobox.Control>
	<Portal>
		<Combobox.Positioner>
			<Combobox.Content>
				{#each result.data as item (item.id)}
					<Combobox.Item {item}>
						<UserCard profile={item} />
					</Combobox.Item>
				{/each}
			</Combobox.Content>
		</Combobox.Positioner>
	</Portal>
</Combobox>
