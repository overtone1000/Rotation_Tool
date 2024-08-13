<script lang="ts">
    import { mdiArrowDown, mdiArrowRight } from "@mdi/js";
    import IconButton, { Icon } from '@smui/icon-button';
	
    export let name:string;
    export let show_children=false;
</script>

<div class="outer_container">
    <div class="top">
        <IconButton on:click={() => {}} toggle bind:pressed={show_children}>
            <Icon tag="svg" viewBox="0 0 24 24" on>
                <path fill="currentColor" d={mdiArrowDown} />
            </Icon>
            <Icon tag="svg" viewBox="0 0 24 24">
                <path fill="currentColor" d={mdiArrowRight} />
            </Icon>
        </IconButton>
        {name}
    </div>
    {#if show_children}
        <slot name="contents"/>
    {/if}
</div>

<style>    
    .top {
        display:flex;
        flex-direction: row;
        justify-content: left;
        align-items: center;
    }
    .outer_container {
        display:flex;
        flex-direction: column;
        flex-grow: 1;
        flex-shrink: 1;
        min-height: 0px;
    }
</style>