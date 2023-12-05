<script lang="ts">
  import Button from '@smui/button';
  import IconButton, { Icon } from '@smui/icon-button';
  import LayoutGrid, { Cell } from '@smui/layout-grid';
  import { Label } from '@smui/list';
  import { BF_Label } from '../../commons/refactored/commons/constants';
  import { BinaryNode, type BNData } from '../../commons/refactored/data/BinaryNode';
  import { DataType } from '../../commons/refactored/data_types';
  import { Constraint } from '../../commons/refactored/extended_types/bndata/Constraint';
  import Dialog from "../../commons/svelte_components/dialog.svelte";
  import DynamicInput from "../../commons/svelte_components/input_elements/dynamic.svelte";
  import Picklist from "../../commons/svelte_components/input_elements/picklist.svelte";
  import type { EditElementProps, PicklistElementProps } from '../../commons/svelte_components/input_elements/props';
  import Modal from '../../commons/svelte_components/modal.svelte';
  
  export let node:BinaryNode;
  export let update:()=>void;

  let unhidden_children:BinaryNode[]=[];
  let remove_handler:(()=>void)=()=>{};
  let change_order_handler:((change:number)=>void)=()=>{};
  let add_handler:(()=>void)=()=>{};
  
  const onSave = () => {instantiator_open=false;console.debug("Save");}
  const onDiscard = () => {instantiator_open=false;console.debug("Discard");}

  let main:any=null;
  let props:EditElementProps;
  let has_instantiable_children:boolean=false;
  let is_reorderable_child:boolean=false;
  let index:number=-1;
  let childcount:number=0;
  
  let instantiator_open=false;
  let instantiator_node:BinaryNode;

  $: {   
    main=null;
    props = {} as EditElementProps;

    //Need to replicate getInputMembers
    if(node)
    {
      const node_data_instance=node.node_data; //This needs to be here to rerender the element in case of a data update.

      props.label = node.getLabel();
      props.type = node_data_instance.d;
      has_instantiable_children=false;
      is_reorderable_child=false;
      instantiator_open=false;
      
      const parent_node = node.parent_node;
      if(parent_node)
      {
        if (
          (node_data_instance.l == BF_Label.AssignmentMember ||
          node_data_instance.l == BF_Label.ConstraintMember)
          ) 
        {
          props.label = props.label + ' ' + node.local_key;
        }

        //If this is an instantiable child, needs deletion and order change handlers
        remove_handler=()=>{};
        change_order_handler=()=>{};
        add_handler=()=>{};
        if (parent_node.node_data.i !== undefined && !parent_node.node_data.r) {
          index = parent_node.node_data.c!.indexOf(node_data_instance);
          childcount = parent_node.node_data.c!.length;
          if(index>=0)
          {
            remove_handler = () => {
              parent_node.node_data.c!.splice(index, 1);            

              //Need special handling for Assignment Member removal as NodeReference datatype will need to change.
              //This needs to happen in the UI, not the backend.
              //If the following is true, this is an assignment member of a schedule template, so adjust indices
              //If parent_node.parent_node is null, this is actually a staging match one constraint, so just skip this
              if (
                node_data_instance.l == BF_Label.AssignmentMember &&
                parent_node.parent_node
              ) {
                const details_node = parent_node.parent_node;
                for (const child_index in details_node.children) {
                  const child = details_node.children[child_index];
                  if (child.node_data.l == BF_Label.ConstraintMembers) {
                    for (
                      let n = child.node_data.c!.length - 1;
                      n >= 0;
                      n-- //Go backwards for safe splicing
                    ) {
                      const constraint_node = child.node_data.c![n];
                      const constraint = new Constraint(constraint_node);
                      const constraint_requires_deletion =
                        constraint.details.handleAssignmentDeletion(index);
                      if (constraint_requires_deletion) {
                        console.debug('Splicing out constraint ' + n);
                        child.node_data.c!.splice(n, 1);
                      }
                    }
                    console.debug('New constraint list', child.node_data.c!);
                  }
                }
              }
              update();              
            }

            change_order_handler = (change: number) => {
              const index = parent_node.node_data.c!.indexOf(node_data_instance);
              const swap_with = index + change;
              if (swap_with >= 0 && swap_with < parent_node.node_data.c!.length) {
                const this_member = parent_node.node_data.c![index];
                const swap_member = parent_node.node_data.c![swap_with];
                parent_node.node_data.c![index] = swap_member;
                parent_node.node_data.c![swap_with] = this_member;

                //Need special handling for Assignment Member removal as NodeReference datatype will need to change.
                //This needs to happen in the UI, not the backend.
                //If the following is true, this is an assignment member of a schedule template, so adjust indices
                //If parent_node.parent_node is null, this is actually a staging match one constraint, so just skip this
                if (
                  node_data_instance.l == BF_Label.AssignmentMember &&
                  parent_node.parent_node !== null
                ) {
                  const details_node = parent_node.parent_node;
                  if(details_node)
                  {
                    for (const child_index in details_node.children) {
                      const child = details_node.children[child_index];
                      if (child.node_data.l == BF_Label.ConstraintMembers) {
                        for (const constraint_node of child.node_data.c!) {
                          const constraint = new Constraint(constraint_node);
                          constraint.details.handleAssignmentIndexSwap(index, swap_with);
                        }
                      }
                    }
                  }
                }
              }
              update();
            };
          };
        }
      }
        
      //Remaining getInputMembers that hasn't been transcribed
      {
        //If node has its own value, need to popuate a main input field
        if (node_data_instance.v!==undefined) { //DO NOT check for null here, as this is used by instantiators!
          props.value=node_data_instance.v;
          
          //Node has a defined value
          switch (node_data_instance.d) {
            case DataType.Binary:
              console.debug('Unhandled binary type with a defined value. Should never happen?');
              break;
            case DataType.Enum:
              {
                main=Picklist;
                const castprops = props as PicklistElementProps;
                castprops.option_labels = node.column_meta.translators[node_data_instance.t!];
                const torders = node.column_meta.translator_orders;
                if (torders) {
                  castprops.option_order = torders[node_data_instance.t!];
                }
              }
              break;
            case DataType.NodeReference:
              {                  
                main=Picklist;
                const castprops = props as PicklistElementProps;
                castprops.option_labels = {};

                let referenced_node: BNData = node.getTopNode().node_data.c![0];
                
                if(referenced_node)
                {
                  for (const child_key in referenced_node.c) {
                    const child_index=parseInt(child_key);
                    const child = referenced_node.c[child_index].c![0];

                    let translator=undefined;
                    const torders = node.column_meta.translator_orders
                    if(child.t)
                    {
                      translator=node.column_meta.translators[child.t];
                      if (torders) {
                        castprops.option_order = torders[child.t];
                      }
                    }

                    if(child && child.v)
                    {
                      if (translator) {
                        const translated_child_value = translator[child.v];
                        castprops.option_labels[child_index] = child_index + ': ' + translated_child_value;
                      } else {
                        console.error('Undefined translator.', child.t, child.v, node.column_meta);
                        castprops.option_labels[child_index] = child_index + ': ' + child.v;
                      }
                    }
                  }
                }
              }
              break;
            case DataType.Array:
            case DataType.DynamicOptionList:
              console.error('Unhandled array type.');
              break;
            default:
              {
                main=DynamicInput;
              }
              break;
          }
        } 
        //This node does not have a defined value
        else {
          has_instantiable_children = (
            node_data_instance.d !== DataType.DynamicOptionList &&
            node_data_instance.i !== undefined
          );

          if(
            node.parent_node &&
            node.parent_node.node_data.i &&
            !(node.parent_node.node_data.r)
          ){
            is_reorderable_child=true;
          }
          
          if(has_instantiable_children)
          {
            const instantiator_index=node.node_data.i;
            if(instantiator_index)
            {
              add_handler=()=>{
                const instantiator = node.column_meta.instantiators[instantiator_index];
                instantiator_node = new BinaryNode(
                  node.table_meta,
                  node.column_meta,
                  JSON.parse(JSON.stringify(instantiator))
                );
                instantiator_open=true;
              }
            }
          }
        }
      }
      //Handles children (like getInputChildren)
      unhidden_children=[];
      for(const child_index in node.children)
      {
        const child_node=node.children[child_index];
        if(!child_node.isHidden())
        {
          unhidden_children.push(child_node);
        }
      }
    }
  }
  console.error("NEED ADD FUNCTION!!! Look to addmember.tsx, function addmember");
</script>

{#if node}
	<div class="outer_container">
    <div class="top">
      <div>{props.label}</div>
      {#if has_instantiable_children}
        <div>
          <IconButton on:click={add_handler} color="primary">
            <Icon class="material-icons">add_circle</Icon>
          </IconButton>
        </div>
      {/if}
    </div>
      <div class="inner_container">
      {#if is_reorderable_child}
        <div class="left">
          {#if index>0 && index<=childcount-1}
            <div>
              <IconButton on:click={() => {change_order_handler(-1)}} color="primary" size="button">
                <Icon class="material-icons">arrow_drop_up</Icon>
              </IconButton>
            </div>
          {/if}
          {#if index>=0 && index<childcount-1}
            <div>
              <IconButton on:click={() => {change_order_handler(+1)}} color="primary" size="button">
                <Icon class="material-icons">arrow_drop_down</Icon>
              </IconButton>
            </div>
          {/if}
        </div>
      {/if}
      <div class="main">
        <svelte:component this={main} {props} />
        <LayoutGrid class="grid">
          {#each unhidden_children as child}
            <Cell class="cell">
              <svelte:self node={child} {update}/>
            </Cell>
          {/each}
        </LayoutGrid>
      </div>
      {#if is_reorderable_child}
        <div class="right">
          <IconButton on:click={remove_handler}>
            <Icon class="material-icons">delete</Icon>
          </IconButton>
        </div>
      {/if}
    </div>
	</div>

  {#if has_instantiable_children}
    <Modal
      zindex={2}
      bind:visible={instantiator_open}
    >
      <Dialog
        title={"New Instance"}
      >
        <div slot="contents">
          <svelte:self node={instantiator_node} {update}/>
        </div>
        <div slot="actions">
          <Button on:click={onDiscard} color="secondary" variant="raised">
            <Label>Discard</Label>
          </Button>
          <Button on:click={onSave} color="primary" variant="raised">
            <Label>Save</Label>
          </Button>
        </div>
      </Dialog>
    </Modal>
  {/if}
{/if}

<style>
	.outer_container {
		border-style: solid;
		border-width: 1px;
		border-color: white;
		border-radius: 5px;
		padding: 3px;
	}
  .inner_container {
    display: flex;
    flex-direction: row;
  }
  .top {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }
  .left {
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
  .right {
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
	:root {
		--mdc-layout-grid-margin-desktop: 3px;
		--mdc-layout-grid-gutter-desktop: 3px;
		/*--mdc-layout-grid-column-width-desktop: 72px;*/
		--mdc-layout-grid-margin-tablet: 3px;
		--mdc-layout-grid-gutter-tablet: 3px;
		/*--mdc-layout-grid-column-width-tablet: 72px;*/
		--mdc-layout-grid-margin-phone: 3px;
		--mdc-layout-grid-gutter-phone: 3px;
		/*--mdc-layout-grid-column-width-phone: 72px;*/
	}
</style>
