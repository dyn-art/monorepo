import React from 'react';
import { ETransformStatus } from '@dyn/figma-to-dtif';

import { appHandler } from '../../app-handler';
import { useAppCallback } from '../../hooks';
import { ToTransformSelection } from './ToTransformSelection';
import { Transforming } from './Transforming';

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
				switch (args.status.type) {
					case ETransformStatus.START:
						setIsTransforming(true);
						break;
					case ETransformStatus.END:
						setTimeout(() => {
							setIsTransforming(false);
						}, 1000);
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
		<>
			<ToTransformSelection isTransforming={isTransforming} />
			{isTransforming && <Transforming />}
		</>
	);
};

export default ToDTIFPlugin;
