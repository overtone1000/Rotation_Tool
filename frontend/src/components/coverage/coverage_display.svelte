<script lang="ts">
	import { getAllCoords, type CoordKeys, type Coordinates, type SiteTree, type TemporalCoverage, type FractionalCoverage, getCoverages, type Coverages } from "./CoverageTree";
	import { day_indices, dowfunc } from '../../commons/time';
	import { onMount } from 'svelte';
	import Select, { Option } from '@smui/select';
	import Drawer, { Content } from "@smui/drawer";
	import FractionalCoverageDisplay from "./fractional_coverage_display.svelte";
    import TemporalCoverageDisplay from "./temporal_coverage_display.svelte";

	let site_tree:SiteTree|undefined=undefined;
    
    let keys:CoordKeys={
        sites: [],
        subspecialties: [],
        contexts: [],
        modalities: []
    }

    let active_coords:Coordinates={
        site: "",
        subspecialty: "General",
        context: "",
        modality: "",
        dow: (new Date()).getDay()
    }

    let coverages:Coverages|undefined=undefined;
    $ : {
        coverages=getCoverages(active_coords,site_tree);
        console.debug("Coverages:",coverages);
    }

	onMount(() => {
		fetch("active_coverage_tree.json").then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:SiteTree)=>{
							site_tree=res;
                            console.debug(site_tree);
                            keys=getAllCoords(site_tree);                         
                            console.debug(keys);
						}
					);
				}
			}
		);
	});

</script>

{#if site_tree !== undefined}
    <div class="container1">
        <Drawer>
            <Content>
                <div class="button_container">
                    <Select
                        label="Site"
                        bind:value={active_coords.site}
                        >
                        {#each keys.sites as o}
                            <Option value={o}>{o}</Option>
                        {/each}
                    </Select>
                </div>
                <div class="button_container">
                    <Select 
                        label="Subspecialty"
                        bind:value={active_coords.subspecialty}
                        >
                        {#each keys.subspecialties as o}
                            <Option value={o}>{o}</Option>
                        {/each}
                    </Select>
                </div>
                <div class="button_container">
                    <Select
                        label="Context"
                        bind:value={active_coords.context}
                        >
                        {#each keys.contexts as o}
                            <Option value={o}>{o}</Option>
                        {/each}
                    </Select>
                </div>
                <div class="button_container">
                    <Select
                        label="Modality"
                        bind:value={active_coords.modality}
                        >
                        {#each keys.modalities as o}
                            <Option value={o}>{o}</Option>
                        {/each}
                    </Select>
                </div>
                <div class="button_container">
                    <Select
                        label="Day of the week"
                        key={dowfunc}
                        bind:value={active_coords.dow}
                        >
                        {#each day_indices as di}
                            <Option value={di}>{dowfunc(di)}</Option>
                        {/each}
                    </Select>
                </div>
            </Content>
        </Drawer>
        <div class="container2">
            {#if coverages !== undefined}
                {#if coverages.Temporal !== undefined}
                    {#each coverages.Temporal as temporal_coverage}
                        <TemporalCoverageDisplay coverage={temporal_coverage}/>
                    {/each}
                {/if}
                {#if coverages.Fractional !== undefined}
                    {#each coverages.Fractional as fractional_coverage}
                        <FractionalCoverageDisplay coverage={fractional_coverage}/>
                    {/each}
                {/if}
            {/if}
        </div>
    </div>
{/if}

<style>
    .container1 {
        position: relative;
        display: flex;
        height: 100%;
        overflow: hidden;
        z-index: 0;
    }
	.button_container {
        margin-left:10px;
        margin-top:10px;
    }
    .container2 {
        margin: 5px;
    }
</style>
