import React from 'react';
import type { COMP } from '@dyn/figma-to-dtif';
import { ScrollArea, SpinnerIcon } from '@dyn/ui';

import { appHandler } from '../../../app-handler';
import { useAppCallback } from '../../../hooks';

import './styles.css';

import { copyToClipboard } from '../../../core/utils';
import { useSVGComposition } from './use-svg-composition';

const WIDTH = 364;
const HEIGHT = 256;

export const CompositionPreview: React.FC<TProps> = (props) => {
	const { isTransforming } = props;

	const [dtif, setDTIF] = React.useState<COMP.DTIFComposition | null>(null);
	const { svgContainerRef, isLoading } = useSVGComposition({
		dtif: dtif ?? undefined,
		deps: [isTransforming],
		dimensions: {
			width: WIDTH,
			height: HEIGHT
		}
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
					await copyToClipboard(JSON.stringify(args.content));
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
		<ScrollArea className="border-base-300 mt-2 border">
			<div
				className="preview flex items-center justify-center overflow-hidden"
				style={{ width: WIDTH, height: HEIGHT }}
			>
				{isLoading && (
					<div className="flex flex-grow flex-col items-center justify-center">
						<SpinnerIcon className="h-4 w-4 animate-spin" />
						<p className="mt-2">Loading Preview</p>
					</div>
				)}
				<div ref={svgContainerRef} />
			</div>
		</ScrollArea>
	);
};

interface TProps {
	isTransforming: boolean;
}
