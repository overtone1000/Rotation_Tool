<script lang="ts">
	import type { CoverageTree } from "./CoverageTree";
	import { day_indices, dowfunc } from '../../commons/time';
	import { onMount } from 'svelte';

	let coverage_tree:CoverageTree|undefined=undefined;
	
	onMount(() => {
		fetch("active_coverage_tree.json").then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:CoverageTree)=>{
							coverage_tree=res;
						}
					);
				}
			}
		);
	});

</script>

{#if coverage_tree !== undefined}
	<div class="container1">
		{JSON.stringify(coverage_tree)}
	</div>
{/if}

<style>
	
</style>
