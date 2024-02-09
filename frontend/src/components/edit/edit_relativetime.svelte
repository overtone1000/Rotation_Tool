<script lang="ts">
	import Textfield from "@smui/textfield";
	import { parseTimePeriodString, timePeriodToString, type TimePeriodString, all_dayoffsets, dayoffsetToDisplayString, parseRelativeTimeString, type RelativeTime } from "../manifest/RotationManifest";
    import Select, { Option } from '@smui/select';

    export let relative_time:RelativeTime;

</script>

<div>
    <Textfield bind:value={relative_time.time} type="time" label="Time"/>
    <Select bind:value={relative_time.day} label="Day Offset">
        {#each all_dayoffsets as dayoffset}
            <Option value={dayoffset}>{dayoffsetToDisplayString(dayoffset)}</Option>
        {/each}
    </Select>
</div>

<style>

</style>
