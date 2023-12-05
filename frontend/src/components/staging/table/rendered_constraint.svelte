<script lang="ts">
	import type { RenderedAssignable } from '../../commons/refactored/staging/members/rendered_assignable';
	import {
		TokenType,
		type GenericRenderedConstraint
	} from '../../commons/refactored/staging/members/rendered_constraint';
	import { rc_bg_col, rc_bg_col_hover } from '../../commons/refactored/theming/theme';
	import { stagingProcessedMembers, stagingSelection } from '../stores';
	import { HighlightingStore } from './highlighting';

	export let hover_override: boolean;
	export let constraint: GenericRenderedConstraint;
	export let assignable: RenderedAssignable;

	let icon: string;
	$: {
		if ($stagingProcessedMembers) {
			switch (constraint.getTokenType(assignable, $stagingProcessedMembers)) {
				case TokenType.SingleWorker:
					icon = 'link';
					break;
				case TokenType.MatchOne_One:
					icon = 'adjust';
					break;
				case TokenType.MatchOne_Candidate:
					icon = 'merge_type';
					break;
			}
		}
	}

	const highlighting_store = new HighlightingStore();
	$: {
		highlighting_store.update(constraint);
	}

	function handleClick(event: MouseEvent) {
		$stagingSelection.handleClick(constraint, event);
		event.stopPropagation();
	}
	let hover = false;
	function handleMouseEnter(event: MouseEvent) {
		hover = true;
		hover_override = true;
	}

	function handleMouseLeave(event: MouseEvent) {
		hover = false;
		hover_override = false;
	}

	let bgcol: string;
	$: {
		if (hover) {
			bgcol = rc_bg_col_hover;
		} else {
			bgcol = rc_bg_col;
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="container"
	on:click={handleClick}
	on:mouseenter={handleMouseEnter}
	on:mouseleave={handleMouseLeave}
>
	<div
		bind:this={highlighting_store.highlight_element}
		class="border"
		style="background-color:{bgcol}"
	>
		<div style="z-index:0;" class="material-icons contained">
			{icon}
		</div>
	</div>
</div>

<style>
	:root {
		--radius: 26px;
	}
	.container {
		height: 100%;
		padding-left: 2px;
	}
	.border {
		width: var(--radius);
		height: var(--radius);
		border: 4px;
		border-style: solid;
		border-color: transparent;
		display: flex;
		align-content: center;
		justify-content: center;
		border-radius: 100px;
	}
	.contained {
		align-self: center;
		justify-self: center;
	}
</style>
