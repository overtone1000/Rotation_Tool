<script lang="ts">
	import Button, { Label } from '@smui/button';
	import FormField from '@smui/form-field';
	import Radio from '@smui/radio';
	import { ConstraintClass } from '../../commons/refactored/extended_types/bndata/Constraint';
	import {
		AddType,
		addTypeToString
	} from '../../commons/refactored/staging/data_processing/processing03';
	import type { ConstraintStagingData } from '../../commons/refactored/staging/data_processing/stagingdata';
	import { RenderedConstraint_MatchOne, RenderedConstraint_SingleWorker, type GenericRenderedConstraint } from '../../commons/refactored/staging/members/rendered_constraint';
	import AssignmentTypeSelect from '../../commons/svelte_components/assignment_type_select.svelte';
	import ConstraintClassSelect from '../../commons/svelte_components/constraint_class_select.svelte';
	import DatePicker from '../../commons/svelte_components/date_picker.svelte';
	import ScheduleTemplateTypeSelect from '../../commons/svelte_components/schedule_template_type_select.svelte';
	import { stagingProcessedDates, stagingSelectedDate } from '../stores';
	import ConstraintEditor from './constraint_editor.svelte';
	import './subpanel.css';

	const contexts: AddType[] = [AddType.Assignment, AddType.ScheduleTemplate, AddType.Constraint];

	let selected = AddType.Assignment;

	let selected_constraint_class: ConstraintClass = ConstraintClass.SingleWorker;
	let constraint:GenericRenderedConstraint;

	$: {
		if(!constraint || constraint.getConstraintClass()!==selected_constraint_class)
		{
			const data={
				t:selected_constraint_class
			} as ConstraintStagingData;
			switch(selected_constraint_class)
			{
				case ConstraintClass.MatchOne:
					{
						data.d={
							m:undefined,
							c:[]
						};
						constraint=new RenderedConstraint_MatchOne(-1,data);
					}
					break;
				case ConstraintClass.SingleWorker:
					{
						data.d={
							s:[]
						};
						constraint=new RenderedConstraint_SingleWorker(-1,data);
					}
					break;
				default:
					console.error("Unhandled type.");
			}
			console.debug("New constraint",constraint.getConstraintClass()===selected_constraint_class);
		}
	}
</script>

<div class="label">Staging Context</div>
{#each contexts as context}
	<div class="radio_item">
		<FormField>
			<Radio bind:group={selected} value={context} disabled={false} />
			{addTypeToString(context)}
		</FormField>
	</div>
{/each}
<div class="subform">
	<div class="group">
		<div class="item centered">
			{#if selected == AddType.Assignment}
				<AssignmentTypeSelect assignment_types={$stagingProcessedDates?.assignment_types} />
			{:else if selected == AddType.ScheduleTemplate}
				<ScheduleTemplateTypeSelect
					schedule_template_types={$stagingProcessedDates?.schedule_template_types}
				/>
			{:else if selected == AddType.Constraint}
				<ConstraintClassSelect bind:selected_class={selected_constraint_class} />
			{/if}
		</div>
		{#if selected == AddType.Assignment || selected == AddType.ScheduleTemplate}
			<div class="item centered">
				<DatePicker
					label="Date"
					bind:date={$stagingSelectedDate}
					min={new Date(2020, 1, 1)}
					max={new Date(2099, 1, 1)}
				/>
			</div>
		{:else}
			<ConstraintEditor bind:constraint={constraint}/>
		{/if}
	</div>
	<div class="button">
		<Button variant="raised"><Label>Add</Label></Button>
	</div>
</div>

<style>
	.label {
		margin-top: 20px;
		padding-left: 5px;
		margin-bottom: 5px;
	}
	.radio_item {
		padding-left: 10px;
	}
</style>
