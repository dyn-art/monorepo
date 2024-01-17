'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-composition';
import { Button, CircleIcon, SquareIcon } from '@dyn/ui';
import { useWindowSize } from '@/hooks';

import { Canvas, type TCanvasProps } from './components';

export const BannerEditor: React.FC<TBannerEditorProps> = (props) => {
	const { width, height, dtif } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const size = useWindowSize(); // TODO: Figure out how to define canvas size based on window size

	const zoomIn = () => {
		if (composition) {
			// Adjust these values for zooming in
			const newWidth = width * 0.8;
			const newHeight = height * 0.8;
			composition.renderer[0].setViewBox(newWidth, newHeight);
		}
	};

	const zoomOut = () => {
		if (composition) {
			// Adjust these values for zooming out
			const newWidth = width * 1.2;
			const newHeight = height * 1.2;
			composition.renderer[0].setViewBox(newWidth, newHeight);
		}
	};

	return (
		<div className="flex flex-col items-center justify-center">
			<div style={{ width: width + 4, height: height + 4 }}>
				<Canvas dtif={dtif} height={height} onLoadedComposition={setComposition} width={width} />
			</div>
			<div className="flex w-full flex-row items-center justify-between ">
				<div>TODO</div>
				<div>
					<Button
						onClick={() => {
							console.log({ composition });
							if (composition != null) {
								composition.spawnRectangle({
									x: 10,
									y: 10,
									width: 100,
									height: 100,
									color: [0, 0, 0]
								});
								composition.update();
							}
						}}
						size="icon"
						variant="outline"
					>
						<SquareIcon className="h-4 w-4" />
					</Button>
					<Button size="icon" variant="outline">
						<CircleIcon className="h-4 w-4" />
					</Button>
				</div>
				<div className="flex flex-row gap-2">
					<Button
						onClick={() => {
							zoomOut();
						}}
						variant="outline"
					>
						Preview
					</Button>
					<Button
						onClick={() => {
							zoomIn();
						}}
					>
						Export
					</Button>
				</div>
			</div>
		</div>
	);
};

export type TBannerEditorProps = {
	// TODO:
} & Omit<TCanvasProps, 'onLoadedComposition'>;
