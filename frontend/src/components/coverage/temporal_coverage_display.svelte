<script lang="ts">
	import { dowfunc, minutes_since_midnight_to_time_string } from "../../commons/time";
	import type { TemporalCoverage } from "./CoverageTree";

	export let coverage:TemporalCoverage;
</script>

<div>
    {coverage.rotation} ({dowfunc(coverage.rotation_day).toString()}) {minutes_since_midnight_to_time_string(coverage.start)}-{minutes_since_midnight_to_time_string(coverage.end)}
</div>

<style>

</style>
