<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
		console.debug('Mounting.');
	});
</script>

<div class="vp_fill">
	<div class="page">
		Hi!
	</div>
</div>

<style>
	.vp_fill {
		width: 100vw;
		height: 100vh;
		max-width: 100vw;
		max-height: 100vh;
		overflow: hidden;
	}
	.page {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
	}
</style>
