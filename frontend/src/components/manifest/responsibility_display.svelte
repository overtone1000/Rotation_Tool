<script lang="ts">
	import { time_range_to_string, wet_read_string } from "../../commons/time";
	import type { Responsibility } from "./RotationManifest";
    import MemberOrList from "./member_or_list.svelte";

	export let responsibility:Responsibility;
    export let dow:number;
    export let hide_wetreads:boolean;
    export let hide_volumes:boolean;

    /*const array_or_string_to_string = (array_or_string:string|string[]) => {
        if(typeof array_or_string == "string")
        {
            return array_or_string;
        }
        else
        {
            let retval = "";
            for(let n=0;n<array_or_string.length;n++)
            {
                retval += array_or_string[n];
                if(n!=array_or_string.length-1)
                {
                    retval+=", ";
                }
            }
            return retval;
        }
    }*/

    $:is_just_wetread = responsibility.contexts==wet_read_string || (responsibility.contexts.length==1 && responsibility.contexts[0]==wet_read_string);
    
    let contexts:string|string[];
    $:{
        contexts=responsibility.contexts;
        if(typeof responsibility.contexts == "object" && hide_wetreads)
        {
            contexts=[];
            for(const context of responsibility.contexts)
            {
                if(context!=wet_read_string)
                {
                    contexts.push(context);
                }
            }
        }
        else
        {
            contexts=responsibility.contexts;
        }
    }
</script>

{#if !(hide_wetreads && is_just_wetread)}
    <tr>
        <td><MemberOrList members={responsibility.sites}/></td>
        <td><MemberOrList members={contexts}/></td>
        <td><MemberOrList members={responsibility.exams}/></td>
        <!-- <td><div class="mdc-typography--body1">{modality_string}</div></td> -->
        {#if responsibility.time_periods!==undefined && responsibility.time_periods!==null}
            <td>
                <ul>
                {#each responsibility.time_periods as period}
                    <li>{time_range_to_string(period,dow)}</li>
                {/each}
                </ul>
            </td>
        {:else if responsibility.weekly_fraction !==undefined && responsibility.weekly_fraction !== null}
            <td>
                {responsibility.weekly_fraction*100 + "% of the week's worth"}
            </td>
        {:else}
            <td></td>
        {/if}
        {#if !hide_volumes}
            <td>
                {#if responsibility.volume!==undefined}
                    {#if responsibility.volume.rvu!==null && responsibility.volume.bvu!==null}
                    <div>Average RVU={responsibility.volume.rvu.toFixed(1)}</div>
                    <div>Average BVU={responsibility.volume.bvu.toFixed(0)}</div>
                    {:else}
                        No volume data for this entry
                    {/if}
                {/if}
            </td>
        {/if}
    </tr>
{/if}

<style>
    td{
		border: 1px solid;
		border-collapse: collapse;
        padding: 3px;
        max-width: 200px;
	}
    ul{
        padding-left: 15px;
    }
</style>
