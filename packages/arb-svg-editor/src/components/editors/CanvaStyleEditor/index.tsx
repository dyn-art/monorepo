import React from 'react';
import { type ARB } from '@dyn/arb-dtif';
import { type Artboard } from '@dyn/arb-svg-builder';
import { useSizeCallback } from '@dyn/ui';
import { usePreparedDtif } from '@/hooks';

import { type TEditorProps } from '../Editor';
import { Sidebar, Toolbar, Viewport } from './components';

export const CanvaStyleEditor: React.FC<TEditorProps> = (props) => {
	const { dtif } = props;
	const [artboard, setArtboard] = React.useState<Artboard | null>(null);
	const viewportRef = React.useRef<HTMLDivElement>(null);
	const { data: preparedDtif } = usePreparedDtif(dtif);

	useSizeCallback(
		viewportRef,
		// Not passing the viewport size as prop to the Artboard or in the DTIF
		// because React is kinda slow updating their states
		(size) => {
			artboard?.emitInputEvents('Core', [
				{
					type: 'UpdateArtboardSize',
					size: [size.width, size.height]
				},
				{ type: 'FocusRootNodes' }
			]);
			artboard?.update();
		},
		[artboard]
	);

	return (
		<div className="flex h-full w-full">
			<Sidebar />
			<main className="relative flex flex-1 flex-col overflow-auto">
				<Toolbar />
				<Viewport dtif={preparedDtif} onLoadedArtboard={setArtboard} viewportRef={viewportRef} />
			</main>
		</div>
	);
};
export interface TCanvaStyleEditorProps {
	dtif?: ARB.DtifArtboard;
}
