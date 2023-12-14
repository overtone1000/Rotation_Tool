<script lang="ts">
	import type { FractionalCoverage } from "./CoverageTree";

	export let coverage:FractionalCoverage;
</script>

<div>
    {coverage.rotation}{coverage.rotation_day}{coverage.fraction}
</div>

<style>

</style>
