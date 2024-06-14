import React from 'react';

import { appHandler } from '../../app-handler';
import { useAppCallback } from '../../hooks';
import { ArtboardPreview } from './ArtboardPreview';
import { ToTransformSelection } from './ToTransformSelection';
import { TransformLoadingIndicator } from './TransformLoadingIndicator';

const ToDTIFPlugin: React.FC = () => {
	const [isTransforming, setIsTransforming] = React.useState(false);

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
						setIsTransforming(true);
						break;
					case 'End':
						setIsTransforming(false);
						break;
					default:
					// do nothing
				}
			}
		},
		[]
	);

	// =========================================================================
	// UI
	// =========================================================================

	return (
		<div className="flex h-full w-full flex-col px-4">
			<ToTransformSelection isTransforming={isTransforming} />
			<TransformLoadingIndicator isTransforming={isTransforming} />
			<ArtboardPreview isTransforming={isTransforming} />
		</div>
	);
};

export default ToDTIFPlugin;
