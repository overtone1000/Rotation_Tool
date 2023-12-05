<script lang="ts">
	import { writable } from 'svelte/store';
	import { Staging_Directives, type ASRequestStagingParameters } from "../../commons/refactored/ajax/commands_generic";
	import { MetaInterpret } from "../../commons/refactored/extended_types/UpdateMeta";
	import type { RenderedAssignable } from "../../commons/refactored/staging/members/rendered_assignable";
	import IDBasedAutocomplete from "../../commons/svelte_components/id_based_autocomplete.svelte";
	import { stagingModification } from '../staging';
	import { stagingInit } from "../stores";


    export let ra:RenderedAssignable;
    
    let id_list:number[];
    let selected_worker=writable<number | undefined>(undefined);

    const labelFunction = (id:number)=>{
        if(id)
        {
            return MetaInterpret.getWorkerName(id,$stagingInit?.update_data.meta);
        }
        return "undefined or null id";
    };

    $:console.debug("ra",ra);
    $:console.debug("selected_worker",selected_worker);
    
    $:
    {
        console.debug("ra init");
        if(ra)
        {
            id_list=ra.getCandidates();
            selected_worker.update(
                ()=>{return ra.getAssignedWorker();}
            );
        }
    }

    $:console.debug("selected_worker",selected_worker);
    
    $:
    {
        console.debug("check selected worker change");
        if($selected_worker !== ra.getAssignedWorker())
        {
            const ra_index=ra.getIndex();
            if(ra_index!==undefined)
            {
                const parameters={} as ASRequestStagingParameters;
                parameters.type=Staging_Directives.assign;
                if($selected_worker)
                {
                    parameters.worker_id=$selected_worker;
                }
                else
                {
                    parameters.worker_id=undefined;
                }
                parameters.staging_ids=[ra_index];
                
                //console.debug("Parameters are...",parameters);
                stagingModification(parameters);
            }
        }
    }

//
</script>

{#if id_list}
<IDBasedAutocomplete
    label="Assigned Worker"
    {id_list}
    {labelFunction}
    disabled={$selected_worker!==null}
    bind:selected_id={$selected_worker}
/>
{/if}