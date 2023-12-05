<script lang="ts">
    export let zindex:number;
    export let visible:boolean;
</script>

{#if visible}
    <div class="mask" style="visible:{visible};z-index:{zindex}">
        <div class="contents">
            <slot/>
        </div>
    </div>
{/if}

<style>
    .mask {
        position:fixed;
        top:0;
        left:0;
        height:100vh;
        width:100vw;
        background-color: hsla(0, 0%, 100%, 0.4);
        display:flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }
    .contents {
        height:fit-content;
        width:fit-content;
    }
</style>