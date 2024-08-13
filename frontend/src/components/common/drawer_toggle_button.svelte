<script lang="ts">
	import { mdiArrowLeft, mdiArrowRight } from "@mdi/js";
	import IconButton, { Icon } from "@smui/icon-button";
	export let open: boolean = true;
</script>

<IconButton
	on:click={() => {
		open = !open;
	}}
	pressed={open}
>
	<Icon tag="svg" viewBox="0 0 24 24" on>
		<path fill="currentColor" d={mdiArrowLeft} />
	</Icon>
	<Icon tag="svg" viewBox="0 0 24 24">
		<path fill="currentColor" d={mdiArrowRight} />
	</Icon>
</IconButton>

<style>
</style>
