<script lang="ts">
	import type { RotationManifest } from "../manifest/RotationManifest";
	import Textfield from '@smui/textfield';
	import RotationEditComponent from "./rotation_edit_component.svelte";
	export let manifest:RotationManifest;
</script>

<div>
    <Textfield bind:value={manifest.title} on:change={()=>console.debug(manifest)} label="Manifest Title" />
    {#each manifest.rotation_manifest as rotation}
        <RotationEditComponent bind:rotation={rotation}/>
    {/each}
</div>

<style>

</style>
