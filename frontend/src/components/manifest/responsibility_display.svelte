<script lang="ts">
	import { time_range_to_string } from "../../commons/time";
	import type { Responsibility } from "./RotationManifest";
    import MemberOrList from "./member_or_list.svelte";

	export let responsibility:Responsibility;
    export let dow:number;

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
</script>

<tr>
	<td><MemberOrList members={responsibility.sites}/></td>
    <td><MemberOrList members={responsibility.contexts}/></td>
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
