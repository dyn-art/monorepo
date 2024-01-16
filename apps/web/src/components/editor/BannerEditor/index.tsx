import React from 'react';

import { Canvas, type TCanvasProps } from './Canvas';

export const BannerEditor: React.FC<TBannerEditorProps> = (props) => {
	const { width, height, dtif } = props;

	return (
		<div className="flex items-center justify-center">
			<div style={{ width, height }}>
				<Canvas dtif={dtif} height={height} width={width} />
			</div>
		</div>
	);
};

export type TBannerEditorProps = {
	// TODO:
} & TCanvasProps;
