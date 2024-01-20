import React, { useState, type RefObject } from 'react';
import type { Composition } from '@dyn/svg-composition';

interface TUseMouseViewBoxOptions {
	moveStep?: number;
	zoomFactor?: number;
}

// TODO: handle in Rust?
export function useMouseViewBox(
	composition: Composition,
	targetRef: RefObject<HTMLDivElement>,
	options: TUseMouseViewBoxOptions = {}
): void {
	const { moveStep = 1, zoomFactor = 0.7 } = options;
	const [isDragging, setIsDragging] = useState(false);
	const startPosRef = React.useRef({ x: 0, y: 0 });

	// Handles the start of the dragging action
	const handleMouseDown = React.useCallback(
		(e: MouseEvent) => {
			if (targetRef.current == null || e.button !== 1) return;

			setIsDragging(true);
			startPosRef.current = { x: e.clientX, y: e.clientY };
			targetRef.current.style.cursor = 'grabbing';
		},
		[targetRef]
	);

	// Handles the dragging movement
	const handleMouseMove = React.useCallback(
		(e: MouseEvent) => {
			if (!isDragging || !targetRef.current) return;

			const deltaX = e.clientX - startPosRef.current.x;
			const deltaY = e.clientY - startPosRef.current.y;
			const factor = composition.viewBox.width / composition.width;

			// Apply the calculated deltas to the viewBox
			const newViewBox = {
				...composition.viewBox,
				minX: composition.viewBox.minX - deltaX * factor * moveStep,
				minY: composition.viewBox.minY - deltaY * factor * moveStep
			};
			composition.updateViewBox(newViewBox);

			// Reset the start position for next movement
			startPosRef.current = { x: e.clientX, y: e.clientY };
		},
		[isDragging, composition, moveStep, startPosRef.current, targetRef.current]
	);

	// Handles the end of the dragging action
	const handleMouseUp = React.useCallback(
		(e: MouseEvent) => {
			if (targetRef.current == null || e.button !== 1) return;

			setIsDragging(false);
			targetRef.current.style.cursor = '';
		},
		[targetRef.current]
	);

	// Handles zoom functionality
	const handleMouseWheel = React.useCallback(
		(e: WheelEvent) => {
			if (!targetRef.current) return;

			if (e.metaKey || e.ctrlKey) {
				e.preventDefault();

				const scaleFactor = e.deltaY < 0 ? 1 / zoomFactor : zoomFactor;
				const rect = targetRef.current.getBoundingClientRect();

				const cursorX = e.clientX - rect.left;
				const cursorY = e.clientY - rect.top;

				const relativeX =
					(cursorX / rect.width) * composition.viewBox.width + composition.viewBox.minX;
				const relativeY =
					(cursorY / rect.height) * composition.viewBox.height + composition.viewBox.minY;

				const newWidth = composition.viewBox.width * scaleFactor;
				const newHeight = composition.viewBox.height * scaleFactor;
				const newMinX = relativeX - (cursorX / rect.width) * newWidth;
				const newMinY = relativeY - (cursorY / rect.height) * newHeight;

				composition.updateViewBox({
					...composition.viewBox,
					minX: newMinX,
					minY: newMinY,
					width: newWidth,
					height: newHeight
				});
			}
		},
		[composition, zoomFactor, targetRef.current]
	);

	// Registers and cleans up event listeners
	React.useEffect(() => {
		const targetElement = targetRef.current;
		if (!targetElement) return;

		targetElement.addEventListener('pointerdown', handleMouseDown);
		targetElement.addEventListener('pointermove', handleMouseMove);
		targetElement.addEventListener('pointerup', handleMouseUp);
		targetElement.addEventListener('pointerleave', handleMouseUp);
		targetElement.addEventListener('wheel', handleMouseWheel);

		return () => {
			targetElement.removeEventListener('pointerdown', handleMouseDown);
			targetElement.removeEventListener('pointermove', handleMouseMove);
			targetElement.removeEventListener('pointerup', handleMouseUp);
			targetElement.removeEventListener('pointerleave', handleMouseUp);
			targetElement.removeEventListener('wheel', handleMouseWheel);
		};
	}, [handleMouseDown, handleMouseMove, handleMouseUp, handleMouseWheel, targetRef]);
}
