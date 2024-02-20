<script lang="ts">
	import RotationsByWeekday from "./rotations_by_weekday.svelte";

	
</script>

<div class="container">
	<RotationsByWeekday />
</div>

<style>
	.container {
		display: flex;
		flex-direction:column;
		height: 100%;
		width: 100%;
		overflow-y: auto;
	}
</style>