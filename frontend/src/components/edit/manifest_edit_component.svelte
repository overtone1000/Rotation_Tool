<script lang="ts">
	import type { RotationManifest } from "../manifest/RotationManifest";
	import Textfield from '@smui/textfield';
	import RotationEditComponent from "./rotation_edit_component.svelte";
    import CollapsibleMember from "./collapsible_member.svelte";
	export let manifest:RotationManifest;
</script>

<div class="outer">
    <div>
        <Textfield bind:value={manifest.title} on:change={()=>console.debug(manifest)} label="Manifest Title" />
    </div>
    <div class="container middle">
        <CollapsibleMember name="Rotations" show_children={true}>
            <div slot="contents" class="inner">
                {#each manifest.rotation_manifest as rotation}
                    <div class="container">
                        <RotationEditComponent bind:rotation={rotation}/>
                    </div>
                {/each}
            </div>
        </CollapsibleMember>
    </div>
</div>

<style>    
    .container {
        border-width: 1px;
        border-style: solid;
        border-radius: 10px;
        margin: 3px;
        padding: 10px;
        flex-shrink: 1;
    }

    .outer {
        display: flex;
        flex-grow: 1;
        flex-direction: column;
        height: 100%;
        max-height: 100%;
        margin-left: 5px;
    }

    .middle {
        display: flex;
        flex-direction: column;
        flex-shrink: 1;
        flex-grow: 1;
        min-height: 0px;
    }

    .inner {
        display: flex;
        flex-grow: 1;
        flex-direction: column;
        min-height: 0px;
        overflow:auto;
    }
</style>