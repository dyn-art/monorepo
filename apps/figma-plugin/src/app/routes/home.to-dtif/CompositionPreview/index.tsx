import React from 'react';
import type { TComposition } from '@dyn/figma-to-dtif';
import { ScrollArea } from '@dyn/ui';

import { appHandler } from '../../../app-handler';
import { useAppCallback } from '../../../hooks';

import './styles.css';

import { useSVGComposition } from './use-svg-composition';

export const CompositionPreview: React.FC<TProps> = (props) => {
	const { isTransforming } = props;
	const [dtif, setDTIF] = React.useState<TComposition | null>(null);

	const { svgContainerRef, composition } = useSVGComposition({
		dtif: dtif ?? undefined,
		deps: [isTransforming]
	});

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
					setDTIF(args.content);
				}
			}
		},
		[]
	);

	// =========================================================================
	// UI
	// =========================================================================

	if (dtif == null || isTransforming) {
		return null;
	}

	return (
		<ScrollArea className="mt-2">
			<div className="relative overflow-x-auto">
				<div className="preview border-base-300 flex min-h-[16rem] w-full items-center justify-center overflow-x-hidden border p-4">
					<div ref={svgContainerRef} />
				</div>
			</div>
		</ScrollArea>
	);
};

interface TProps {
	isTransforming: boolean;
}
