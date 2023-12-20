import React from 'react';
import type { TComposition } from '@dyn/figma-to-dtif';
import { Icons, ScrollArea } from '@dyn/ui';

import { appHandler } from '../../../app-handler';
import { useAppCallback } from '../../../hooks';

import './styles.css';

import { useSVGComposition } from './use-svg-composition';

const WIDTH = 364;
const HEIGHT = 256;

export const CompositionPreview: React.FC<TProps> = (props) => {
	const { isTransforming } = props;

	const [dtif, setDTIF] = React.useState<TComposition | null>(null);
	const { svgContainerRef, composition, isLoading } = useSVGComposition({
		dtif: dtif ?? undefined,
		deps: [isTransforming]
	});

	const [scale, setScale] = React.useState(1);

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

	React.useEffect(() => {
		if (composition != null) {
			const scaleX = WIDTH / composition.width;
			const scaleY = HEIGHT / composition.height;
			const newScale = Math.min(scaleX, scaleY, 1);

			setScale(newScale);
		}
	}, [composition]);

	// =========================================================================
	// UI
	// =========================================================================

	if (dtif == null || isTransforming) {
		return null;
	}

	return (
		<ScrollArea className="border-base-300 mt-2 border">
			<div
				className="preview flex items-center justify-center overflow-hidden p-4"
				style={{ width: WIDTH, height: HEIGHT }}
			>
				{isLoading && (
					<div className="flex flex-grow flex-col items-center justify-center">
						<Icons.spinner className="mr-2 h-4 w-4 animate-spin" />
						<p className="mt-2">Loading Preview</p>
					</div>
				)}
				<div ref={svgContainerRef} style={{ transform: `scale(${scale})` }} />
			</div>
		</ScrollArea>
	);
};

interface TProps {
	isTransforming: boolean;
}
