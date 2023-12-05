<script lang="ts">
	import Checkbox from '@smui/checkbox';
	import FormField from '@smui/form-field';
	import Textfield from '@smui/textfield';
	import { DataType } from '../../refactored/data_types';
	import DatePicker from '../date_picker.svelte';
	import type { EditElementProps } from './props';

	export let props: EditElementProps;

	switch (props.type) {
		case DataType.DisableableDouble:
			console.error('Unhandled!');
		/*
			return (
				<DisableableDoubleControl
					id={id}
					label={label}
					disabled={readonly}
					object={value_object}
					object_key={value_key}
					changeHandler={changeHandler}
				/>
			);
            */
		case DataType.LocalDateAsEpochDay:
			console.error('Unhandled!');
		//return createEpochDayControl(id, label, readonly, value_object, value_key, changeHandler);
	}
</script>

{#if props.type == DataType.String}
	<Textfield type="string" bind:value={props.value} />
{:else if props.type == DataType.Boolean}
	<FormField>
		<Checkbox bind:checked={props.value} />
		<span slot="label">{props.label}</span>
	</FormField>
{:else if props.type == DataType.Float}
	<Textfield type="number" bind:value={props.value} />
{:else if props.type == DataType.Integer || props.type == DataType.Long || props.type == DataType.Enum || props.type == DataType.NodeReference}
	<Textfield type="number" bind:value={props.value} input$step="1" />
{:else if props.type == DataType.LocalTime}
	<Textfield type="time" bind:value={props.value} />
{:else if props.type == DataType.LocalDate}
	<DatePicker label={props.label} date={props.value} />
{:else}
	UNHANDLED TYPE {props.type.toString()}
{/if}
