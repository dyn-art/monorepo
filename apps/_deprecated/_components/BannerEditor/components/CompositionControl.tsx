'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-composition';
import {
	Button,
	ChevronDownIcon,
	ChevronLeftIcon,
	ChevronRightIcon,
	ChevronUpIcon,
	ZoomInIcon,
	ZoomOutIcon
} from '@dyn/ui';

export const CompositionControl: React.FC<TProps> = (props) => {
	const { composition, moveStep = 10, zoomFactor = 0.5 } = props;

	const handleForward = React.useCallback(() => {
		const newViewBox = { ...composition.viewBox, minY: composition.viewBox.minY - moveStep };
		composition.updateViewBox(newViewBox);
	}, [composition, moveStep]);

	const handleLeft = React.useCallback(() => {
		const newViewBox = { ...composition.viewBox, minX: composition.viewBox.minX - moveStep };
		composition.updateViewBox(newViewBox);
	}, [composition, moveStep]);

	const handleRight = React.useCallback(() => {
		const newViewBox = { ...composition.viewBox, minX: composition.viewBox.minX + moveStep };
		composition.updateViewBox(newViewBox);
	}, [composition, moveStep]);

	const handleBack = React.useCallback(() => {
		const newViewBox = { ...composition.viewBox, minY: composition.viewBox.minY + moveStep };
		composition.updateViewBox(newViewBox);
	}, [composition, moveStep]);

	const handleZoomIn = React.useCallback(() => {
		const newViewBox = {
			...composition.viewBox,
			width: composition.viewBox.width * zoomFactor,
			height: composition.viewBox.height * zoomFactor
		};
		composition.updateViewBox(newViewBox);
	}, [composition, zoomFactor]);

	const handleZoomOut = React.useCallback(() => {
		const newViewBox = {
			...composition.viewBox,
			width: composition.viewBox.width / zoomFactor,
			height: composition.viewBox.height / zoomFactor
		};
		composition.updateViewBox(newViewBox);
	}, [composition, zoomFactor]);

	return (
		<div className="grid grid-cols-3 place-items-center gap-4 p-4">
			{/* Placeholder for top-left corner */}
			<div />

			{/* Up */}
			<Button onClick={handleForward} size="icon" variant="outline">
				<ChevronUpIcon className="h-4 w-4" />
			</Button>

			{/* Placeholder for top-right corner */}
			<div />

			{/* Left */}
			<Button onClick={handleLeft} size="icon" variant="outline">
				<ChevronLeftIcon className="h-4 w-4" />
			</Button>

			{/* Down */}
			<Button onClick={handleBack} size="icon" variant="outline">
				<ChevronDownIcon className="h-4 w-4" />
			</Button>

			{/* Right */}
			<Button onClick={handleRight} size="icon" variant="outline">
				<ChevronRightIcon className="h-4 w-4" />
			</Button>

			{/* Placeholder for bottom-left corner */}
			<div />

			{/* Zoom In */}
			<Button onClick={handleZoomIn} size="icon" variant="outline">
				<ZoomInIcon className="h-4 w-4" />
			</Button>

			{/* Zoom Out */}
			<Button onClick={handleZoomOut} size="icon" variant="outline">
				<ZoomOutIcon className="h-4 w-4" />
			</Button>
		</div>
	);
};

interface TProps {
	composition: Composition;
	moveStep?: number;
	zoomFactor?: number;
}
