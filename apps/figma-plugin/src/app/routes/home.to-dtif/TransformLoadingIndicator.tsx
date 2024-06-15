import React from 'react';
import { ETransformStatus } from '@dyn/figma-to-dtif';
import { SpinnerIcon } from '@dyn/ui';

import { appHandler } from '../../app-handler';
import { useAppCallback } from '../../hooks';

const randomFacts = [
	'This is a random fact :)',
	'DTIF means "Design Tree Interchange Format"'
] as const;

export const TransformLoadingIndicator: React.FC<TProps> = (props) => {
	const { isTransforming } = props;
	const [message, setMessage] = React.useState('Loading');
	const [subMessage, setSubMessage] = React.useState<string>(
		randomFacts[Math.floor(Math.random() * randomFacts.length)] ?? randomFacts[0]
	);

	// =========================================================================
	// Lifecycle
	// =========================================================================

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'on-transform-status-update',
			callback: async (instance, args) => {
				switch (args.type) {
					case 'Start':
						setMessage('Getting Ready');
						break;
					case 'Transform': {
						switch (args.status.type) {
							case ETransformStatus.START:
								setMessage('Traversing Figma Node Tree');
								break;
							case ETransformStatus.TRAVERSED_TREE:
								setSubMessage(
									`Transforming ${args.status.toTransformNodesCount} Nodes, ${args.status.toTransformPaintsCount} Paints and  ${args.status.toTransformAssetsCount} Assets`
								);
								break;
							case ETransformStatus.TRANSFORMING_NODES:
								setMessage('Transforming Nodes');
								break;
							case ETransformStatus.TRANSFORMING_PAINTS:
								setMessage('Transforming Paints');
								break;
							case ETransformStatus.TRANSFORMING_ASSETS:
								setMessage('Transforming Assets');
								break;
							case ETransformStatus.CONSTRUCTING_CANVAS:
								setMessage('Constructing Artboard');
								break;
							case ETransformStatus.END:
								setMessage('Completed transforming Frame to DTIF');
								break;
						}
						break;
					}
					case 'Transmit':
						setMessage('Sending DTIF to Figma Plugin Frontend');
						break;
					case 'End':
						setMessage('Preparing');
						break;
				}
			}
		},
		[]
	);

	// =========================================================================
	// UI
	// =========================================================================

	if (!isTransforming) {
		return null;
	}

	return (
		<div className="flex h-full w-full flex-col items-center justify-center">
			<div className="flex flex-grow flex-col items-center justify-center">
				<SpinnerIcon className="mr-2 h-4 w-4 animate-spin" />
				<p className="mt-2">{message}</p>
			</div>
			<p className="mb-2 mt-auto text-sm opacity-80">{subMessage}</p>
		</div>
	);
};

interface TProps {
	isTransforming: boolean;
}
