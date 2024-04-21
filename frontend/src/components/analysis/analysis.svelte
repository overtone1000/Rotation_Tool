<script lang="ts">
	import RotationsByWeekday from "./rotation_volume_heatmap.svelte";

	
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