<script lang="ts">
	import Drawer, { Content as DrawerContent } from "@smui/drawer";
	import List, { Item } from "@smui/list";
	import type { RotationManifest } from "../manifest/RotationManifest";
	import Dialog, { Title, Content, Actions } from '@smui/dialog';
  	import Button, { Label } from '@smui/button';
	import { key } from '../../commons/key';
	import ManifestEditComponent from "./manifest_edit_component.svelte";

	let manifest:RotationManifest|undefined=undefined;
	
	const discard="discard";
	let discard_dialog_open=false;
	let dialogCloseHandler:(result:any)=>void;
	const discard_check=(func:()=>void)=>{
		console.debug("Discard check.");
		if(manifest===undefined)
		{
			console.debug("Manifest is undefined. Running function.");
			func();
		}
		else
		{
			console.debug("Manifest is defined. Opening dialog.");
			discard_dialog_open=true;
			dialogCloseHandler=(result)=>{
				console.debug("Dialog result",result);
				if(result.detail.action===discard){func();}
			}
		}
	}

	const get_current=()=>{
		discard_check(()=>{
				console.debug("Fetching active rotation manifest.");
				fetch("data/active_rotation_manifest"+key+".json").then(
					(value:Response)=>{
						if(value.ok)
						{
							value.json().then(
								(res:RotationManifest)=>{
									manifest=res;
									console.debug("Manifest is now ",manifest);
								},
								(err)=>{
									console.error("Rejected promise.",err);
								}
							);
						}
					},
					(err)=>{
						console.error("Rejected fetch",err);
					}
				);
			}
		);
	};

	let uploaded_json:FileList|undefined=undefined;
	$ : {
		console.debug("Updated uploaded JSON");
		if(uploaded_json!==undefined)
		{
			const file = uploaded_json.item(0);
			if(file!==null && file!==undefined)
			{
				file.text().then(
					(str)=>{
						let parsed = JSON.parse(str);
						if(parsed!==null && parsed!==undefined)
						{
							discard_check(()=>{
								manifest=parsed;
							});
						}
					}
				)
			}
		}
	}

	const download=()=>{
		let data = JSON.stringify(manifest);
		let file = new Blob([data], {type:"application/json"});
		const a = document.createElement("a"),
                url = URL.createObjectURL(file);
        a.href = url;
		let dt = new Date();
        a.download = manifest?.title+" (" + 
			dt.getFullYear().toString().padStart(2,"0") + "-" + 
			dt.getMonth().toString().padStart(2,"0") + "-" +
			dt.getDate().toString().padStart(2,"0") + " " + 
			dt.getHours().toString().padStart(2,"0") + 
			dt.getMinutes().toString().padStart(2,"0") + 
			dt.getSeconds().toString().padStart(2,"0") + ")"
			".json";
        document.body.appendChild(a);
        a.click();
        setTimeout(function() {
            document.body.removeChild(a);
            window.URL.revokeObjectURL(url);  
        }, 0); 
	}
</script>

<Dialog
  bind:open={discard_dialog_open}
  aria-labelledby="event-title"
  aria-describedby="event-content"
  on:SMUIDialog:closed={dialogCloseHandler}
>
  <Title id="event-title">Discarding Current Draft</Title>
  <Content id="event-content">
    Do you want to discard the current draft?
  </Content>
  <Actions>
    <Button action={discard}>
      <Label>Yes</Label>
    </Button>
    <Button defaultAction>
      <Label>No</Label>
    </Button>
  </Actions>
</Dialog>

<div class="container1">
    <div>
        <Drawer>
            <DrawerContent>
                <List>
                    <Item
                        on:click={get_current}
                        >
                        Open Current
                    </Item>
                    <Item>
						<div class="upload_button_label">Upload</div>
						<input class="upload_button" accept="application/json" bind:files={uploaded_json} type="file"/>
                    </Item>
					{#if manifest !== undefined}
						<Item
							on:click={download}
						>
							Download
						</Item>
					{/if}
                </List>
            </DrawerContent>
        </Drawer>		
    </div>
    {#if manifest !== undefined}
		<ManifestEditComponent manifest={manifest}/>
	{/if}
</div>

<style>
    .container1 {
		display: flex;
		flex-direction:row;
		height: 100%;
	}
	.upload_button_label {
		z-index:-1;
	}
	.upload_button {
		opacity: 0%;
		width: 100%;
		height: 100%;
		position:fixed;
		left:-5px;
		cursor:inherit;
	}
</style>
