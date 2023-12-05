import { FC } from 'react';
import { INodeFabProps } from '../BinaryNode';
import { assembleNode } from './common';

export const GenericInputNode: FC<INodeFabProps> = (props: INodeFabProps) => {
	return assembleNode({ props: props, child_node_width: 'auto' });
};
