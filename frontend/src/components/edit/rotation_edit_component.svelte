<script lang="ts">
	import type { Rotation } from "../manifest/RotationManifest";
    import Textfield from '@smui/textfield';
	export let rotation:Rotation;
</script>

<div>
    <Textfield bind:value={rotation.rotation} label="Name"/>
    <Textfield bind:value={rotation.location} label="Location"/>
    Hours, Breaktime, Responsibilities, Comments
</div>

<style>

</style>
