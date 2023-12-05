<script lang="ts">
	import Button, { Label } from '@smui/button';
	import type { BinaryNode } from '../../commons/refactored/data/BinaryNode';
	import Dialog from "../../commons/svelte_components/dialog.svelte";
	import Modal from "../../commons/svelte_components/modal.svelte";
	import BinaryNodeChild from './binary_node_child.svelte';

	export let open: boolean;
	export let title: string;
	export let parent_node: BinaryNode;

	//let labeledby = 'dialog-title' + Math.random().toString();
	//let describedby = 'dialog-content' + Math.random().toString();

	const onSave = () => {
		console.debug('Save.');
		open=false;
	};
	const onDiscard = () => {
		console.debug('Discard.');
		open=false;
	};

	const update = () => {
		parent_node=parent_node.clone();
	};
</script>

<Modal
	zindex={2}
	bind:visible={open}
>
	<Dialog
		title={title}
	>
		<div slot="contents">
			<BinaryNodeChild node={parent_node} {update}/>
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