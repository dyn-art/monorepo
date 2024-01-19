import React, { useState, type RefObject } from 'react';

interface TUseMouseViewBoxOptions {
	moveStep?: number;
	zoomFactor?: number;
}

export function useMouseViewBox(
	composition: Composition,
	targetRef: RefObject<HTMLDivElement>,
	options: TUseMouseViewBoxOptions = {}
): void {
	const { moveStep = 10, zoomFactor = 0.5 } = options;
	const [isDragging, setIsDragging] = useState(false);
	const [startPos, setStartPos] = useState({ x: 0, y: 0 });

	const handleMouseDown = React.useCallback(
		(e: MouseEvent) => {
			if (targetRef.current == null) {
				return;
			}

			if (e.button === 1) {
				// Middle mouse button
				setIsDragging(true);
				setStartPos({ x: e.clientX, y: e.clientY });
				targetRef.current.style.cursor = 'grabbing'; // Change cursor to hand icon
			}
		},
		[targetRef]
	);

	const handleMouseMove = React.useCallback(
		(e: MouseEvent) => {
			if (!isDragging) return;

			const deltaX = e.clientX - startPos.x;
			const deltaY = e.clientY - startPos.y;
			setStartPos({ x: e.clientX, y: e.clientY });

			const newViewBox = {
				...composition.viewBox,
				minX: composition.viewBox.minX - deltaX * moveStep,
				minY: composition.viewBox.minY - deltaY * moveStep
			};
			composition.updateViewBox(newViewBox);
		},
		[composition, isDragging, startPos, moveStep]
	);

	const handleMouseUp = React.useCallback(
		(e: MouseEvent) => {
			if (targetRef.current == null) {
				return;
			}

			if (e.button === 1) {
				// Middle mouse button
				setIsDragging(false);
				targetRef.current.style.cursor = ''; // Restore cursor
			}
		},
		[targetRef]
	);

	const handleMouseWheel = React.useCallback(
		(e: WheelEvent) => {
			if (targetRef.current == null) {
				return;
			}

			if (e.metaKey || e.ctrlKey) {
				// Prevent the default scroll behavior
				e.preventDefault();

				// Calculate the scale factor
				const scaleFactor = e.deltaY < 0 ? 1 / zoomFactor : zoomFactor;

				// Get the bounding rectangle of the target element
				const rect = targetRef.current.getBoundingClientRect();

				// Calculate the cursor's position relative to the composition
				const cursorX = e.clientX - rect.left;
				const cursorY = e.clientY - rect.top;

				// Convert the cursor position to the composition's coordinate space
				const relativeX =
					(cursorX / rect.width) * composition.viewBox.width + composition.viewBox.minX;
				const relativeY =
					(cursorY / rect.height) * composition.viewBox.height + composition.viewBox.minY;

				// Adjust the viewBox properties for zooming around the cursor
				const newWidth = composition.viewBox.width * scaleFactor;
				const newHeight = composition.viewBox.height * scaleFactor;
				const newMinX = relativeX - (cursorX / rect.width) * newWidth;
				const newMinY = relativeY - (cursorY / rect.height) * newHeight;

				// Update the viewBox
				const newViewBox = {
					...composition.viewBox,
					minX: newMinX,
					minY: newMinY,
					width: newWidth,
					height: newHeight
				};
				composition.updateViewBox(newViewBox);
			}
		},
		[composition, targetRef, zoomFactor]
	);

	React.useEffect(() => {
		const targetElement = targetRef.current;
		if (targetElement) {
			targetElement.addEventListener('mousedown', handleMouseDown);
			targetElement.addEventListener('mousemove', handleMouseMove);
			window.addEventListener('mouseup', handleMouseUp); // Global listener to handle mouse up anywhere
			targetElement.addEventListener('wheel', handleMouseWheel);
		}

		return () => {
			if (targetElement) {
				targetElement.removeEventListener('mousedown', handleMouseDown);
				targetElement.removeEventListener('mousemove', handleMouseMove);
				window.removeEventListener('mouseup', handleMouseUp);
				targetElement.removeEventListener('wheel', handleMouseWheel);
			}
		};
	}, [handleMouseDown, handleMouseMove, handleMouseUp, handleMouseWheel, targetRef]);
}

interface Composition {
	id: string;
	viewBox: ViewBox;
	updateViewBox: (newViewBox: ViewBox) => void;
}

interface ViewBox {
	minX: number;
	minY: number;
	width: number;
	height: number;
}
