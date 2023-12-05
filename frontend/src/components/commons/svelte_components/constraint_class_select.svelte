<script lang="ts">
	import { ConstraintClass } from '../refactored/extended_types/bndata/Constraint';
	import IDBasedSelect from './id_based_select.svelte';

	export let selected_class: ConstraintClass;

	const types: ConstraintClass[] = [];
	types.push(ConstraintClass.SingleWorker);
	types.push(ConstraintClass.MatchOne);

	function labelFunction(cclass: ConstraintClass) {
		switch (cclass) {
			case ConstraintClass.SingleWorker:
				return 'Single Worker';
			case ConstraintClass.MatchOne:
				return 'Match One';
			default:
				return 'unahndled';
		}
	}
</script>

<IDBasedSelect
	label="Assignment Type"
	id_list={types}
	{labelFunction}
	bind:selected_id={selected_class}
/>
