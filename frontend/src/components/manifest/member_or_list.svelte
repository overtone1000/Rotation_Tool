<script lang="ts">
	export let members:string|string[];

    const array_or_string_to_array = (array_or_string:string|string[]) => {
        if(typeof array_or_string == "string")
        {
            return [array_or_string];
        }
        else
        {
            return array_or_string
        }
    }

    $:members_array=array_or_string_to_array(members);
</script>

{#if members_array.length==1}
    {members_array[0]}
{:else if members_array.length>1}
    <ul>
        {#each members_array as member}
            <li>{member}</li>
        {/each}
    </ul>
{/if}

<style>
    ul{
        padding-left: 15px;
    }
</style>
