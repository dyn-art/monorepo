import React from 'react';
import { Composition } from '@dyn/svg-composition';

import { SelectionBox } from './components';

export const CanvasControl: React.FC<TProps> = (props) => {
	const { composition } = props;

	return (
		<svg
			id={'canvas-control'}
			className="pointer-events-none absolute h-full w-full"
			overflow={'visible'}
		>
			<SelectionBox
				composition={composition}
				onResizeHandlePointerDown={() => {
					// TODO
				}}
			/>
		</svg>
	);
};

type TProps = {
	composition: Composition;
};
