<script lang="ts">
	import type { Responsibility } from "./RotationManifest";

	export let responsibility:Responsibility;

    const array_or_string_to_string = (array_or_string:string|string[]) => {
        console.debug(array_or_string,typeof array_or_string == "string")
        if(typeof array_or_string == "string")
        {
            return array_or_string;
        }
        else
        {
            let retval = "";
            for(let n=0;n<array_or_string.length;n++)
            {
                retval += array_or_string[n];
                console.debug(array_or_string[n]);
                if(n!=array_or_string.length-1)
                {
                    retval+="/";
                }
            }
            return retval;
        }
    }

    let site_string = array_or_string_to_string(responsibility.sites);
    let context_string = array_or_string_to_string(responsibility.contexts);
    let modality_string = array_or_string_to_string(responsibility.modalities);
    let subspecialty_string = array_or_string_to_string(responsibility.subspecialties);
    
    let portion:string = "";
    if (responsibility.time_periods !== undefined && responsibility.time_periods !== null)
    {
        portion = array_or_string_to_string(responsibility.time_periods);
    }

    if (responsibility.weekly_fraction !==undefined && responsibility.weekly_fraction !== null)
    {
        portion = responsibility.weekly_fraction*100 + "% of the week's worth";
    }

</script>

<div>
	<div class="mdc-typography--body1">
		{site_string}
        {subspecialty_string}
        {context_string}
        {modality_string}
        {responsibility.days.toString()}
        {portion}
	</div>
</div>

<style>

</style>
