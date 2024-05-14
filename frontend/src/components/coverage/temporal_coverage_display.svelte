<script lang="ts">
	import { dowfunc, minutes_since_midnight_to_time_string, shortdowfunc } from "../../commons/time";
	import type { TemporalCoverage } from "./CoverageTree";

	export let coverage:TemporalCoverage;
	export let day:number;

	$:dowstring=shortdowfunc(day)+" ";

	let dowrotation:number=0;
	$:{
		dowrotation=day+coverage.work_to_rotation_day_offset;
		while(dowrotation>6)
		{
			dowrotation-=7;
		}
	}
</script>

<tr>
    <td>{coverage.rotation}</td>
	<td>{dowfunc(dowrotation).toString()}</td>
	<td>{dowstring+minutes_since_midnight_to_time_string(coverage.start)}</td>
	<td>{dowstring+minutes_since_midnight_to_time_string(coverage.end)}</td>
</tr>

<style>
	td{
		border: 1px solid;
		border-collapse: collapse;
	}
</style>
