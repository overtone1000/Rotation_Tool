<script lang="ts">
	import { get_data, type BuiltPlot } from "./commons";
    import ObservablePlot from "./plot_scaffold.svelte";

    type DataFormat = $$Generic;
    type MarkFormat = $$Generic;
    type PlotOptions = $$Generic;

	export let filename:string;
    export let plot_options:PlotOptions;
    export let get_marks:(data:DataFormat)=>MarkFormat[];
    export let build_plot:(marks:MarkFormat[],width:number,options:PlotOptions)=>BuiltPlot;
        
    let data:DataFormat|undefined = undefined;
    let marks:MarkFormat[]|undefined;
    let plot:BuiltPlot|undefined = undefined;
    let container_width:any="5000px";

    get_data(filename,(result:any)=>{data=result as DataFormat});
    
    $:{
        if(data!==undefined)
        {
            marks=get_marks(data);
        }
    }

    $:{
        if(marks!==undefined)
        {
            plot=build_plot(marks,container_width,plot_options);
        }
    }
</script>

<div class="plot_container" bind:clientWidth={container_width}>
    {#if plot!==undefined}
        <ObservablePlot plot={plot}/>
    {/if}
</div>

<style>
	.plot_container {
		width:100%;
	}
</style>
