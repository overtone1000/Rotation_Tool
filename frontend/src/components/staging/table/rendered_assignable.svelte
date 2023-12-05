<script lang="ts">
	import type { Unsubscriber } from 'svelte/store';

	import { onDestroy } from 'svelte';
	import type { DataMeta } from '../../commons/needs_refactoring/data_processing/data_types';
	import { MetaInterpret } from '../../commons/refactored/extended_types/UpdateMeta';
	import type { RenderedAssignable } from '../../commons/refactored/staging/members/rendered_assignable';
	import type { GenericRenderedConstraint } from '../../commons/refactored/staging/members/rendered_constraint';
	import { RABackgroundColors } from '../../commons/refactored/theming/theme';
	import { stagingInit, stagingProcessedMembers, stagingSelection } from '../stores';
	import { HighlightingStore } from './highlighting';
	import RenderedConstraint from './rendered_constraint.svelte';

	export let assignable: RenderedAssignable;
	export let type_top: boolean;
	export let priority_top: boolean;

	let worker_name = '';
	let background = 'black';
	$: {
		//Set background color
		if (assignable && $stagingInit) {
			const meta = $stagingInit.update_data.meta as DataMeta;

			let color_selection_hover;
			if (hover && !hover_override) {
				color_selection_hover = RABackgroundColors.hover;
			} else {
				color_selection_hover = RABackgroundColors.default;
			}

			let color_selection_locked;
			if (assignable.isLocked()) {
				color_selection_locked = color_selection_hover.locked;
			} else {
				color_selection_locked = color_selection_hover.unlocked;
			}

			if (assignable.getAssignedWorker()) {
				worker_name = MetaInterpret.getWorkerName(assignable.getAssignedWorker(), meta);
				background = color_selection_locked.assigned;
			} else {
				background = color_selection_locked.unassigned;
			}
		}
	}

	const highlighting_store = new HighlightingStore();
	$: {
		highlighting_store.update(assignable);
	}

	let all_constraints: GenericRenderedConstraint[] = [];
	let static_constraint_arr: GenericRenderedConstraint[] = [];
	let proposed_constraint_arr: GenericRenderedConstraint[] = [];
	let constraints_store_subscription: Unsubscriber;
	$: {
		if (assignable) {
			if (constraints_store_subscription) {
				constraints_store_subscription();
			}

			const constraints_store = assignable.getConstraintsStore();

			const process_constraints = (constraints: IterableIterator<GenericRenderedConstraint>) => {
				const retval = [];
				for (const constraint of constraints) {
					retval.push(constraint);
				}
				return retval;
			};

			constraints_store_subscription = constraints_store.subscribe(
				(constraints: Map<number,GenericRenderedConstraint>) => {
					static_constraint_arr = process_constraints(constraints.values());
				}
			);
		}
	}

	$: {
		const cset = new Set<GenericRenderedConstraint>();
		for (const c of static_constraint_arr) {
			cset.add(c);
		}
		for (const c of proposed_constraint_arr) {
			cset.add(c);
		}
		all_constraints = [];
		cset.forEach((c) => {
			if ($stagingProcessedMembers) {
				const tokentype = c.getTokenType(assignable, $stagingProcessedMembers);
				if (tokentype !== undefined) {
					all_constraints.push(c);
				}
			}
		});
	}

	function handleClick(event: MouseEvent) {
		console.debug('Handleclick outside');
		$stagingSelection.handleClick(assignable, event);
		event.stopPropagation();
	}

	let hover = false;
	let hover_override = false;
	function handleMouseEnter(event: MouseEvent) {
		hover = true;
	}

	function handleMouseLeave(event: MouseEvent) {
		hover = false;
	}

	onDestroy(() => {
		if (constraints_store_subscription) {
			constraints_store_subscription();
		}
		highlighting_store.unsubscribe();
	});
</script>

<td class:type_top class:priority_top>
	{#if assignable}
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			bind:this={highlighting_store.highlight_element}
			class="assignable"
			style="background:{background};"
			on:click={handleClick}
			on:mouseenter={handleMouseEnter}
			on:mouseleave={handleMouseLeave}
		>
			<div class="left">
				{#each all_constraints as constraint}
					<RenderedConstraint {constraint} {assignable} bind:hover_override />
				{/each}
			</div>
			<div class="right">
				<div class="assignment_type">
					<b>{assignable.getAssignableType()?.getName()}</b>
				</div>
				<div class="worker">
					<i>{worker_name}</i>
				</div>
			</div>
		</div>
	{/if}
</td>

<style>
	td {
		/* 
        border: 1px;
        border-color: purple;
        */
		border-style: none;
		user-select: none;
	}

	td.type_top {
		border-top: 1px;
		border-top-style: solid;
		border-top-color: hsl(0, 0%, 20%);
	}

	td.priority_top {
		border-top: 1px;
		border-top-style: double;
		border-top-color: hsl(0, 0%, 80%);
	}

	div.left {
		align-self: center;
		align-items: center;
		display: flex;
		flex-direction: row;
		flex-shrink: 1;
		overflow-x: auto;
	}

	div.right {
		align-self: center;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		flex-grow: 1;
		flex-shrink: 0;
	}

	div.assignment_type {
		height: 50%;
	}

	div.worker {
		color: rgb(252, 177, 0);
		min-height: 50%;
	}

	.assignable {
		border: 4px;
		border-style: solid;
		border-color: transparent;
		height: 40px;
		padding: 1px;
		padding-right: 15px;
		border-radius: 20px;
		display: flex;
		flex-direction: row;
		align-items: flex-start;
	}
</style>
