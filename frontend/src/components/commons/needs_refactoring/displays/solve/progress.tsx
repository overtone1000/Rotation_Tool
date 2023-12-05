import { Box } from '@mui/material';
import React, { FC } from 'react';

interface FullfillmentDisplayProps {
	min: number;
	max: number;
}

const clamp = (value: number, min: number, max: number) => {
	return Math.max(min, Math.min(value, max));
};

const clamp_0_1 = (value: number) => {
	return clamp(value, 0, 1);
};

export const FulfillmentDisplay: FC<FullfillmentDisplayProps> = (
	props: FullfillmentDisplayProps
) => {
	let min = clamp_0_1(props.min);
	let max = clamp_0_1(props.max);

	const div1 = min * 100.0;
	const div2 = max * 100.0;

	const width_1 = div1 + '%';
	const width_2 = div2 - div1 + '%';
	const width_3 = 100 - div2 + '%';

	console.debug('Fullfillment Display', props, min, max, div1, div2, width_1, width_2, width_3);

	return (
		<Box
			style={{ margin: '0px', padding: '0px', display: 'flex', flexDirection: 'row' }}
			width="100%"
			height="5px"
		>
			<Box
				width={width_1}
				height="100%"
				style={{ flexShrink: 0, flexGrow: 0, backgroundColor: 'green' }}
			/>
			<Box
				width={width_2}
				height="100%"
				style={{ flexShrink: 0, flexGrow: 0, backgroundColor: 'yellow' }}
			/>
			<Box
				width={width_3}
				height="100%"
				style={{ flexShrink: 0, flexGrow: 0, backgroundColor: 'red' }}
			/>
		</Box>
	);
};
