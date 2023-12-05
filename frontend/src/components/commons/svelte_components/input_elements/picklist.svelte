<script lang="ts">
	import Select, { Option } from '@smui/select';
	import type { PicklistElementProps } from './props';

	export let props: PicklistElementProps;

	const disabled_map = new Map<number, boolean>();
	for (const option_key in props.option_labels) {
		const option_index = parseInt(option_key);
		if (props.option_disabled_indices && props.option_disabled_indices.includes(option_index)) {
			disabled_map.set(option_index, false);
		} else {
			disabled_map.set(option_index, true);
		}
	}
</script>

<Select bind:value={props.value}>
	<!-- label={props.label}> leaving out label because of rendering problem. Can't figure it out.-->
	{#each props.option_order as option_index}
		<Option value={option_index} disabled={disabled_map.get(option_index)}>
			{props.option_labels[option_index]}
		</Option>
	{/each}
</Select>
