import React from 'react';
import { JSONTree } from 'react-json-tree';
import { ScrollArea } from '@dyn/ui';

import { appHandler } from '../app-handler';
import { ClipboardButton } from '../components/primitive/ClipboardButton';
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
		<ScrollArea className="flex h-full w-full border bg-[#090300]">
			<div className="relative w-full p-4">
				<ClipboardButton
					toCopy={JSON.stringify(selectedNodes)}
					className="absolute right-0 top-0 z-50 m-4"
				/>
				<JSONTree data={selectedNodes} theme={threezerotwofourTheme} />
			</div>
		</ScrollArea>
	);
};

export default NodeInspectorPlugin;
