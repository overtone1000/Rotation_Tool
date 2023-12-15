<script lang="ts">
	import { dowfunc, minutes_since_midnight_to_time_string, shortdowfunc } from "../../commons/time";
	import type { TemporalCoverage } from "./CoverageTree";

	export let coverage:TemporalCoverage;
	export let day:number;

	$:dowstring=shortdowfunc(day)+" ";
</script>

<tr>
    <td>{coverage.rotation}</td>
	<td>{dowfunc(coverage.rotation_day).toString()}</td>
	<td>{dowstring+minutes_since_midnight_to_time_string(coverage.start)}</td>
	<td>{dowstring+minutes_since_midnight_to_time_string(coverage.end)}</td>
</tr>

<style>
	td{
		border: 1px solid white;
		border-collapse: collapse;
	}
</style>
