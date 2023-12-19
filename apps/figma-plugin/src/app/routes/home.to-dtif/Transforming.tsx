import React from 'react';
import { ETransformStatus } from '@dyn/figma-to-dtif';
import { Icons } from '@dyn/ui';

import { appHandler } from '../../app-handler';
import { useAppCallback } from '../../hooks';

export const Transforming: React.FC = () => {
	const [message, setMessage] = React.useState('Loading');
	const [subMessage, setSubMessage] = React.useState<string>('TODO: Random facts here');

	// =========================================================================
	// Lifecycle
	// =========================================================================

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'on-transform-status-update',
			callback: async (instance, args) => {
				switch (args.status.type) {
					case ETransformStatus.START:
						setMessage('Traversing Figma Node Tree');
						break;
					case ETransformStatus.TRAVERSED_TREE:
						setSubMessage(
							`Transforming ${args.status.toTransformNodesCount} Nodes, ${args.status.toTransformPaintsCount} Paints and  ${args.status.toTransformFontsCount} Fonts`
						);
						break;
					case ETransformStatus.TRANSFORMING_NODES:
						setMessage('Transforming Nodes');
						break;
					case ETransformStatus.TRANSFORMING_PAINTS:
						setMessage('Transforming Paints');
						break;
					case ETransformStatus.TRANSFORMING_FONTS:
						setMessage('Transforming Fonts');
						break;
					case ETransformStatus.CONSTRUCTING_COMPOSITON:
						setMessage('Constructing Composition');
						break;

					default:
					// do nothing
				}
			}
		},
		[]
	);

	return (
		<div className="flex h-full w-full flex-col items-center justify-center">
			<div className="flex flex-grow flex-col items-center justify-center">
				<Icons.spinner className="mr-2 h-4 w-4 animate-spin" />
				<p className="mt-2">{message}</p>
			</div>
			<p className="mt-auto text-sm opacity-80">{subMessage}</p>
		</div>
	);
};
