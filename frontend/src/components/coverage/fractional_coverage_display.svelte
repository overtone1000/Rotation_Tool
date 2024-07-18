<script lang="ts">
	import { dowfunc } from "../../commons/time";
	import type { FractionalCoverage } from "./CoverageTree";

	export let coverage: FractionalCoverage;
</script>

<tr>
	<td>{coverage.rotation}</td>
	<td>{dowfunc(coverage.rotation_day)}</td>
	<td>{coverage.fraction * 100 + "%"}</td>
</tr>

<style>
	td {
		border: 1px solid;
		border-collapse: collapse;
	}
</style>
