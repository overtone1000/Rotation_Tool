<script lang="ts">
    import Button, { Label } from '@smui/button';
    export let label:string;
    export let newMember:()=>void;
</script>

<div>
    <slot/>
    <div class="button_container">
        <Button style="height:100%; width:100%;" on:click={newMember} variant="raised">
            <Label>New {label}</Label>
        </Button>
    </div>
</div>

<style>    
    .button_container{
        width: 100%-5px;
        height: 40px;
        margin: 5px;
    }
</style>