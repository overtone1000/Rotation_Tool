<script lang="ts">
    export let title:string;
</script>

<div class="container">
    <div class="title"><b>{title}</b></div>
    <div class="contents">
        <slot name="contents"/>
    </div>
    <div class="actions">
        <slot name="actions"/>
    </div>
</div>

<style>
    .container {
        background-color: black;
        height:fit-content;
        width:fit-content;
        padding:10px;
        border-radius:10px;
    }
    .title {
        margin-bottom:10px;
    }
    .actions {
        margin-top:10px;
        display:flex;
        flex-direction: row;
        align-content: flex-end;
        justify-content: flex-end;
    }
</style>