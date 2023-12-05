<script lang="ts">
	import Select, { Option } from '@smui/select';

	export let label: string;
	export let id_list: number[];
	export let labelFunction: (id: number) => string;
	export let selected_id: number;
</script>

{#if id_list}
	<Select bind:value={selected_id} {label} variant="outlined">
		{#each id_list as option}
			<Option value={option}>
				{labelFunction(option)}
			</Option>
		{/each}
	</Select>
{/if}
