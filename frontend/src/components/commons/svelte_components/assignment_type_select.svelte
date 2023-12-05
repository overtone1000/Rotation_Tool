<script lang="ts">
	import type { AssignmentTypeTable } from '../refactored/extended_types/id_tables/AssignmentTypeTable';
	import IDBasedAutocomplete from './id_based_autocomplete.svelte';

	export let assignment_types: AssignmentTypeTable | undefined;
	let assignment_type_ids: number[] = [];
	let selected_type: number|undefined = undefined; //Autocmoplete doesn't act right with undefined, use null
	if (assignment_types) {
		assignment_type_ids = assignment_types.getIDs();
		//selected_type=assignment_type_ids[0];
	}

	const labelFunction = (id: number) => {
		if (assignment_types) {
			console.debug("Types are",assignment_types.getTypes());
			const type = assignment_types.getType(id);
			if (type) {
				return type.getName();
			}
		}
		return "undefined " + id.toString();
	};
	console.debug('Assignment Type Select', assignment_types);
</script>

{#if assignment_types}
	<IDBasedAutocomplete
		label="Assignment Type"
		id_list={assignment_type_ids}
		{labelFunction}
		bind:selected_id={selected_type}
	/>
{/if}
