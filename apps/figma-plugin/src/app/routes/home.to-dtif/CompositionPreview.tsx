import React from 'react';
import type { TComposition } from '@dyn/figma-to-dtif';

import { appHandler } from '../../app-handler';
import { useAppCallback } from '../../hooks';

export const CompositionPreview: React.FC<TProps> = (props) => {
	const { isTransforming } = props;
	const [composition, setComposition] = React.useState<TComposition | null>(null);

	// =========================================================================
	// Lifecycle
	// =========================================================================

	useAppCallback(
		appHandler,
		{
			type: 'plugin.message',
			key: 'intermediate-format-export-result',
			callback: async (instance, args) => {
				if (args.type === 'success') {
					setComposition(args.content);
				}
			}
		},
		[]
	);

	// =========================================================================
	// UI
	// =========================================================================

	if (composition == null || isTransforming) {
		return null;
	}

	return (
		<>
			<p>{JSON.stringify(composition)}</p>
		</>
	);
};

interface TProps {
	isTransforming: boolean;
}
