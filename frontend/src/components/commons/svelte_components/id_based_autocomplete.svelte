<script lang="ts">
	import Autocomplete from '@smui-extra/autocomplete';
	import { writable } from 'svelte/store';
				
	export let label: string;
	export let id_list: number[];
	export let labelFunction: (id: number) => string;
	export let selected_id: number|undefined = undefined;
	export let disabled:boolean = false;

	//Just using numbers probably doesn't work because 0 returns false in comparisons.
	
	const objectified_list:{i:number}[] = [];
	for(const id of id_list)
	{
		objectified_list.push({i:id});
	}
	console.debug("Objectified list",objectified_list);

	const stringfunc = (o:{i:number}) => {
		if(o && o.i !== undefined && o.i !==null)
		{
			return labelFunction(o.i);
		}
		else
		{
			return "";
		}
	}

	let selected_value=writable<{i:number|undefined}>({i:selected_id});		

	//Update selected value if selected id is changed
	$: selected_value.update(
		()=>{return {i:selected_id}}
	);

	//Update selected id if selected value is changed
	$: {
		if($selected_value)
		{
			selected_id=$selected_value.i;
		}
		else
		{
			selected_id=undefined;
		}
	}

</script>

{#if id_list}
	<Autocomplete 
	style="width: 100%;"
    textfield$style="width: 100%;"
	options={objectified_list}
	getOptionLabel={stringfunc}
	{disabled}
	textfield$variant="outlined"
	bind:value={$selected_value}
	{label}
	/>
{/if}