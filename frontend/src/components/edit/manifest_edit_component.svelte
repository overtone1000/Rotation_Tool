<script lang="ts">
	import type { RotationManifest } from "../manifest/RotationManifest";
	import Textfield from '@smui/textfield';
	import RotationEditComponent from "./rotation_edit_component.svelte";
    import CollapsibleMember from "./collapsible_member.svelte";
	import EditArray from "./edit_array.svelte";
	
    export let manifest:RotationManifest;

    const newRotation = () => {
        manifest.rotation_manifest.push(
            {
                rotation:"New Rotation",
                location:"",
                responsibilities:[],
                comments:[]
            }
        );

        manifest.rotation_manifest=manifest.rotation_manifest; //Force component update with assignment
    }
</script>

<div class="outer">
    <div>
        <Textfield bind:value={manifest.title} on:change={()=>console.debug(manifest)} label="Manifest Title" />
    </div>
    <div class="rounded_borders middle">
        <CollapsibleMember name="Rotations" show_children={true}>
            <div slot="contents" class="inner">
                <EditArray label="Rotation" newMember={newRotation}>
                    {#each manifest.rotation_manifest as rotation}
                        <div class="rounded_borders">
                            <RotationEditComponent bind:rotation={rotation}/>
                        </div>
                    {/each}
                </EditArray>
            </div>
        </CollapsibleMember>
    </div>
</div>

<style>    
    
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