<script lang="ts">
	import { nd, pbd, pbdp1, pd, shortdowfunc, time_range_to_string } from "../../commons/time";
	import type { Responsibility } from "./RotationManifest";

	export let responsibility:Responsibility;
    export let dow:number;

    const array_or_string_to_string = (array_or_string:string|string[]) => {
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
    }

    let site_string = array_or_string_to_string(responsibility.sites);
    let context_string = array_or_string_to_string(responsibility.contexts);
    let modality_string = array_or_string_to_string(responsibility.modalities);
    let subspecialty_string = array_or_string_to_string(responsibility.subspecialties);
</script>

<tr>
	<td><div class="mdc-typography--body1">{site_string}</div></td>
    <td><div class="mdc-typography--body1">{subspecialty_string}</div></td>
    <td><div class="mdc-typography--body1">{context_string}</div></td>
    <td><div class="mdc-typography--body1">{modality_string}</div></td>
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
</tr>

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
