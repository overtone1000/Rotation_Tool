<script lang="ts">
	import { parseTimePeriodString, timePeriodToString, type TimePeriodString, all_dayoffsets, dayoffsetToDisplayString, parseRelativeTimeString, type RelativeTime } from "../manifest/RotationManifest";
    import SveltyPicker, { config } from 'svelty-picker';
    import Select, { Option } from '@smui/select';

    export let relative_time:RelativeTime;
</script>

<div>
    <SveltyPicker mode=time bind:value={relative_time.time} format="hh:ii"/>
    <Select bind:value={relative_time.day} label="Day Offset">
        {#each all_dayoffsets as dayoffset}
            <Option value={dayoffset}>{dayoffsetToDisplayString(dayoffset)}</Option>
        {/each}
    </Select>
</div>

<style>
    :root {
        --sdt-bg-main: #585858;
        --sdt-shadow-color: #777;
        --sdt-color: #eee;
        --sdt-clock-color: var(--sdt-color);
        --sdt-clock-color-hover: var(--sdt-color);
        --sdt-clock-time-bg: transparent;
        --sdt-clock-time-bg-hover: transparent;
        --sdt-clock-disabled: #b22222;
        --sdt-clock-disabled-bg: var(--sdt-bg-main);
        --sdt-clock-selected-bg: var(--sdt-bg-selected);
        --sdt-header-color: #eee;
        --sdt-bg-selected: #e1ac4a;
        --sdt-table-disabled-date: #b22222;
        --sdt-table-disabled-date-bg: var(--sdt-bg-main);
        --sdt-table-data-bg-hover: #777;
        --sdt-table-selected-bg: var(--sdt-bg-selected);
        --sdt-header-btn-bg-hover: #777;
        --sdt-color-selected: #fff;
        --sdt-table-today-indicator: #ccc;
        --sdt-clock-bg: #999;
        /* custom buttons */
        --sdt-today-bg: #e4a124;
        --sdt-today-color: #fff;
        --sdt-clear-color: #666;
        --sdt-clear-bg: #ddd;
        --sdt-clear-hover-color: #fff;
        --sdt-clear-hover-bg: #dc3545;
    }
</style>
