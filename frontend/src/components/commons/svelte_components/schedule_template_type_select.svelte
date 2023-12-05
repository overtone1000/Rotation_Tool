<script lang="ts">
	import type { ScheduleTemplateTable } from '../refactored/extended_types/id_tables/ScheduleTemplateTable';
	import IDBasedAutocomplete from './id_based_autocomplete.svelte';

	export let schedule_template_types: ScheduleTemplateTable | undefined;
	let schedule_template_ids: number[] = [];
	let selected_type: number|undefined = undefined;
	if (schedule_template_types) {
		schedule_template_ids = schedule_template_types.getIDs();
		console.debug("Schedule template types",schedule_template_types,schedule_template_ids);
	}

	const labelFunction = (id: number) => {
		if (schedule_template_types) {
			const type = schedule_template_types.getType(id);
			if (type) {
				return type.getName();
			}
		}
		return "undefined " + id.toString();
	};

	console.debug(
		'Schedule Template Type Select',
		schedule_template_ids,
		selected_type,
		schedule_template_types
	);
</script>

{#if schedule_template_types}
	<IDBasedAutocomplete
		label="Schedule Template Type"
		id_list={schedule_template_ids}
		{labelFunction}
		bind:selected_id={selected_type}
	/>
{:else}
	<div />
{/if}
