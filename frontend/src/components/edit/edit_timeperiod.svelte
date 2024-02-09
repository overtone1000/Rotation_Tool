<script lang="ts">
	import { parseTimePeriodString, timePeriodToString, type TimePeriodString, all_dayoffsets, dayoffsetToDisplayString } from "../manifest/RotationManifest";
    import Select, { Option } from '@smui/select';
    import EditRelativeTime from "./edit_relativetime.svelte"

    export let period:TimePeriodString;
    let parsed_period=parseTimePeriodString(period);

    //Change period when parsed_period is adjusted.
    $:{
        period=timePeriodToString(parsed_period);
    }
</script>

<div class="container">
    <slot name="label"/>
    <EditRelativeTime bind:relative_time={parsed_period.start}/>
    <EditRelativeTime bind:relative_time={parsed_period.end}/>
</div>

<style>
    .container {
        display:flex;
        flex-direction: row;
        align-items: center;
    }
</style>
