<script lang="ts">
	import { nd, pbd, pbdp1, pd, shortdowfunc } from "../../commons/time";
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

    const relative_time_to_string = (relative_time:string) => {
        const split = relative_time.split(" ");

        let new_dow:number;
        switch(split[1])
        {
            case "ND":new_dow=nd(dow);
                break;
            case "CD":new_dow=dow;
                break;
            case "PD":new_dow=pd(dow);
                break;
            case "PBD":new_dow=pbd(dow);
                break;
            case "PBD+1":new_dow=pbdp1(dow);
                break;
            default:new_dow=dow;
        }
        let daystring:string=shortdowfunc(new_dow);

        return split[0] + " " + daystring;
    }

    const time_range_to_string = (time_range:string) => {
        const split = time_range.split("-");

        return relative_time_to_string(split[0])+" - "+relative_time_to_string(split[1]);
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
                <li>{time_range_to_string(period)}</li>
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
