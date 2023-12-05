import React from 'react';
import { RectBounds } from './RectBounds';

const svg_uri = 'http://www.w3.org/2000/svg';

interface ReactSVG_props {
	bounds: RectBounds;
	children: React.ReactNode[];
}

export const ReactSVG: React.FC<ReactSVG_props> = (props) => {
	const width = props.bounds.right - props.bounds.left;
	const height = props.bounds.bottom - props.bounds.top;
	const viewbox = '0 0 ' + width + ' ' + height;
	return (
		<svg
			viewBox={viewbox}
			xmlns={svg_uri}
			preserveAspectRatio="xMidYMid meet"
			width={width}
			height={height}
		>
			{props.children}
		</svg>
	);
};
