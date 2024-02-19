<script lang="ts">
	import type * as Plot from "@observablehq/plot";

	export let plot:(HTMLElement | SVGSVGElement) & Plot.Plot;
	
	let plot_div:HTMLDivElement;

	$:{
		plot_div?.firstChild?.remove(); // remove old chart, if any
    	plot_div?.append(plot); // add the new chart
	}
</script>

<div class="plot_div" bind:this={plot_div} role="img"></div>

<style>
	.plot_div {
	}
</style>
