<script lang="ts">
	import List, { Item } from '@smui/list';
	import { Content } from '@smui/paper';

	import { onMount } from 'svelte';
	import { toDateString_ShortDisplay } from '../../commons/refactored/commons/Dates';
	import type { RenderedAssignable } from '../../commons/refactored/staging/members/rendered_assignable';
	import { ConstraintMemberSelectorColors } from '../../commons/refactored/theming/theme';
	import './subpanel.css';

	export let label: string;
	export let members: RenderedAssignable[];
	export let selected = false;
	export let onClick: (event: MouseEvent) => void;

	let color: string = ConstraintMemberSelectorColors.default_color;
	const setColor = (hover: boolean) => {
		if (hover) {
			color = ConstraintMemberSelectorColors.mouseover_color;
		} else {
			if (selected) {
				color = ConstraintMemberSelectorColors.selected_color;
			} else {
				color = ConstraintMemberSelectorColors.default_color;
			}
		}
	};

	let border_color: string;
	$: {
		if (selected) {
			border_color = 'white';
		} else {
			border_color = 'transparent';
		}
	}

	const mouseEnter = (event: MouseEvent) => {
		setColor(true);
	};

	const mouseLeave = (event: MouseEvent) => {
		setColor(false);
	};

	const onClickInternal = (event: MouseEvent) => {
		onClick(event);
		setColor(true);
	};

	onMount(() => {
		setColor(false);
	});
</script>

<div class="subform">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="group bordered variable_border"
		style="background-color:{color}; border-color:{border_color}"
		on:mouseenter={mouseEnter}
		on:mouseleave={mouseLeave}
		on:click={onClickInternal}
	>
		<div class="label">{label}</div>
		<Content>
			<List nonInteractive>
				{#each members as member}
					<Item>
						<div class="item_div bordered">
							<div>{toDateString_ShortDisplay(member.getDate())}</div>
							<div>{member.getAssignableType()?.getName()}</div>
						</div>
					</Item>
				{/each}
			</List>
		</Content>
	</div>
</div>

<style>
	.item_div {
		width: 100%;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		border-style: solid;
		border-color: lightblue;
		border-width: 1px;
		padding: 6px;
		margin: 0px;
		pointer-events: none;
		user-select: none;
	}
	.bordered {
		border-radius: 8px;
	}
	.variable_border {
		border-style: solid;
		border-width: 3px;
	}
	.label {
		text-align: center;
		margin-top: 5px;
	}
</style>
