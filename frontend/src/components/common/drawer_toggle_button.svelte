<script lang="ts">
	import IconButton, { Icon } from "@smui/icon-button";
    export let open:boolean=true;
</script>

<IconButton
    on:click={() => {open = !open;}}
    pressed={open}
>
    <Icon class="material-icons" on>keyboard_arrow_left</Icon>
    <Icon class="material-icons">keyboard_arrow_right</Icon>
</IconButton>

<style>
    
</style>