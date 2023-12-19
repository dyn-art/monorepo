import React from 'react';
import { JSONTree } from 'react-json-tree';

import { appHandler } from '../app-handler';
import { useAppCallback } from '../hooks';
import threezerotwofourTheme from '../styles/json-tree/threezerotwofour.theme';

const NodeInspectorPlugin: React.FC = () => {
	const [selectedNodes, setSelectedNodes] = React.useState<SceneNode[]>([]);

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'on-select-node-properties',
			callback: async (instance, args) => {
				setSelectedNodes(args.selected);
			}
		},
		[]
	);

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'on-change-selected-node-properties',
			callback: async (instance, args) => {
				const { changed } = args;
				const updatedNodeIndex = selectedNodes.findIndex((node) => node.id === changed.id);
				if (updatedNodeIndex != -1) {
					const newSelectedNodes = [...selectedNodes];
					newSelectedNodes[updatedNodeIndex] = changed;
					setSelectedNodes(newSelectedNodes);
				}
			}
		},
		[selectedNodes]
	);

	return (
		<div className="flex w-full overflow-x-hidden border bg-[#090300]">
			<div className="relative w-full overflow-x-auto p-4">
				<JSONTree data={selectedNodes} theme={threezerotwofourTheme} />
			</div>
		</div>
	);
};

export default NodeInspectorPlugin;
