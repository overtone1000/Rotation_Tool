<script lang="ts">
	import { getAllCoords, type CoordKeys, type Coordinates, type SiteTree, type TemporalCoverage, type FractionalCoverage, getCoverages, type Coverages } from "./CoverageTree";
	import { day_indices, dowfunc } from '../../commons/time';
	import { onMount } from 'svelte';
	import Select, { Option } from '@smui/select';
	import Drawer, { Content } from "@smui/drawer";
	import FractionalCoverageDisplay from "./fractional_coverage_display.svelte";
    import TemporalCoverageDisplay from "./temporal_coverage_display.svelte";
	import DrawerToggleButton from "../common/drawer_toggle_button.svelte";
	import type { ExamCategory } from "./ExamCategory";
	import FormField from "@smui/form-field";
    import Switch from '@smui/switch';
    import Autocomplete from '@smui-extra/autocomplete';

	let site_tree:SiteTree|undefined=undefined;
    let exam_categories:ExamCategory[]|undefined=undefined;
    
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
    }

	onMount(() => {
		fetch("active_coverage_tree.json").then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:SiteTree)=>{
							site_tree=res;
                            keys=getAllCoords(site_tree);                         
						}
					);
				}
			}
		);

        fetch("exam_categories.json").then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:ExamCategory[])=>{
							exam_categories=res;                  
                            console.debug("Exam categories",exam_categories);
						}
					);
				}
			}
		);
	});

    let open=true;
    let search_by_exam_description=false;

    let getExamLabel = (option:ExamCategory) => {
        if(option===undefined || option===null){return "";}
        return option.exam;
    }

    let examSelected = (selection:ExamCategory) =>
    {
        active_coords.subspecialty=selection.subspecialty;
    }
</script>

{#if site_tree !== undefined}
    <div class="container1">
        <div class="drawer" hidden={!open}>
            <Drawer>
                <Content>
                    <div class="button_container">
                        <FormField enabled>
                            <span slot="label">Search by exam description.</span>
                            <Switch color="primary" disabled={exam_categories===undefined} bind:checked={search_by_exam_description} />
                        </FormField>
                    </div>
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
                        {#if search_by_exam_description && exam_categories!==undefined}
                            <Autocomplete
                                options={exam_categories} 
                                getOptionLabel={getExamLabel}
                                on:Select={examSelected}
                                label="Exam Description"
                            />
                        {:else}
                            <Select 
                                label="Subspecialty"
                                bind:value={active_coords.subspecialty}
                                >
                                {#each keys.subspecialties as o}
                                    <Option value={o}>{o}</Option>
                                {/each}
                            </Select>
                        {/if}
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
        </div>
        <div class="container2">
            <DrawerToggleButton bind:open={open}/>
            {#if coverages !== undefined}
            <table class="tablecont">
                {#if coverages.Temporal !== undefined}
                    <tr><th>Rotation</th><th>Rotation Day</th><th>Start Time</th><th>End Time</th></tr>
                    {#each coverages.Temporal as temporal_coverage}
                        <TemporalCoverageDisplay coverage={temporal_coverage} day={active_coords.dow}/>
                    {/each}
                {/if}
                {#if coverages.Fractional !== undefined}
                    <tr><th>Rotation</th><th>Rotation Day</th><th>Week %</th></tr>
                    {#each coverages.Fractional as fractional_coverage}
                        <FractionalCoverageDisplay coverage={fractional_coverage}/>
                    {/each}
                {/if}
            </table>
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
        display:flex;
        flex-direction: column;
        width:100%;
    }
    .tablecont
    {
        flex-shrink:1;
        overflow:scroll;
        border: 1px solid;
		border-collapse: collapse
    }
    th{
		border: 1px solid;
		border-collapse: collapse;
    }
</style>
