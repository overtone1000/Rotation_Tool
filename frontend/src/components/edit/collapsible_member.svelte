<script lang="ts">
    import IconButton, { Icon } from '@smui/icon-button';
	
    export let name:string;
    let show_children=false;
</script>

<div>
    <div class="top">
        <IconButton on:click={() => {}} toggle bind:pressed={show_children}>
            <Icon class="material-icons" on>arrow_drop_down</Icon>
            <Icon class="material-icons">arrow_right</Icon>
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
        align-content: center;
    }
</style>