<script lang="ts">
	import type { Rotation } from "../manifest/RotationManifest";
	import CollapsibleMember from "./collapsible_member.svelte";
    import EditTimePeriod from "./edit_timeperiod.svelte";
    import EditWeekdays from "./edit_weekdays.svelte";
    import Textfield from '@smui/textfield';
	export let rotation:Rotation;
</script>

<CollapsibleMember name={rotation.rotation} show_children={false}>
    <div slot="contents" class="inner">
        <Textfield bind:value={rotation.rotation} label="Name"/>
        <Textfield bind:value={rotation.location} label="Location"/>
        {#if rotation.hours}
            <div class="rounded_borders">
                Hours
                {#each rotation.hours as period}
                    <EditTimePeriod bind:period={period.hours}/>
                    <EditWeekdays bind:weekdays={period.days}/>
                {/each}            
            </div>
        {/if}
        {#if rotation.breaktime}
            <div class="rounded_borders">
                Break
                <EditTimePeriod bind:period={rotation.breaktime[0]}/>
                <Textfield style="width: 100%;" bind:value={rotation.breaktime[1]} label="Coverage"/>
            </div>
        {/if}
        
        Responsibilities, Comments
    </div>
</CollapsibleMember>

<style>

</style>
