import React from 'react';

import { appHandler } from '../app-handler';
import { useAppCallback } from '../hooks';

const NodeInspectorPlugin: React.FC = () => {
	const [selectedNodes, setSelectedNodes] = React.useState<SceneNode[]>([]);

	useAppCallback(appHandler, {
		type: 'plugin.message',
		key: 'on-select-node-properties',
		callback: async (instance, args) => {
			const selected = args.selected;
			if (selected.length > 0) {
				setSelectedNodes(selected);
			} else {
				setSelectedNodes([]);
			}
		}
	});

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
		<>
			<p>Node Inspector</p>
		</>
	);
};

export default NodeInspectorPlugin;
