import React from 'react';

import { Canvas, type TCanvasProps } from '../../Canvas';
import { ToolsBar } from './ToolsBar';

export const Viewport: React.FC<TViewportProps> = (props) => {
	return (
		<>
			<Canvas {...props} />
			<ToolsBar />
		</>
	);
};

export type TViewportProps = {
	// TODO:
} & TCanvasProps;
