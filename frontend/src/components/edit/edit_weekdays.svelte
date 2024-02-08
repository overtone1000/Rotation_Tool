<script lang="ts">
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import type { Weekday } from "../manifest/RotationManifest";
    import { all_weekdays } from "../manifest/RotationManifest";

    export let weekdays:Weekday[];

    
    let value:Weekday[];
</script>

<div>
    <FormField>
        {#each all_weekdays as wd}
            <Checkbox
                bind:group={weekdays}
                value={wd}
            />
            {wd}
        {/each}
    </FormField>
</div>

<style>

</style>
