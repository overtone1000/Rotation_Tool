import type { DataType } from '../../refactored/data_types';

export interface EditElementProps {
	label: string;
	value: any;
	type: DataType;
}

export interface PicklistElementProps extends EditElementProps {
	value: number;
	option_labels: { [i: number]: string };
	option_order: number[];
	option_disabled_indices: number[];
	disabled: boolean;
	allownull: boolean;
}
