import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';
import { CURSOR, useEntity, useInteractionMode, useViewportFactor, type TComponent } from '@/hooks';

import { ResizeCornerHandle } from './ResizeCornerHandle';
import { ResizeEdgeHandle } from './ResizeEdgeHandle';
import { RotateCornerHandle } from './RotateCornerHandle';

export const EntitySelectionBox: React.FC<TProps> = (props) => {
	const {
		artboard,
		entity,
		onResizeHandlePointerDown,
		onResizeHandlePointerUp,
		onRotateHandlePointerDown,
		onRotateHandlePointerUp
	} = props;
	const { Size: sizeData, GlobalTransform: globalTransformData } = useEntity(
		artboard,
		entity,
		['Size', 'GlobalTransform'],
		true
	);
	const factor = useViewportFactor(artboard);
	const factoredSizeData = React.useMemo<TComponent<'Size'> | null>(
		() =>
			sizeData != null
				? { size: [sizeData.size[0] * factor[0], sizeData.size[1] * factor[1]] }
				: null,
		[factor, sizeData]
	);
	const factoredTransformData = React.useMemo<TComponent<'Transform'> | null>(
		() =>
			globalTransformData != null
				? {
						translation: [
							globalTransformData.translation[0] * factor[0],
							globalTransformData.translation[1] * factor[1]
						],
						rotationDeg: globalTransformData.rotationDeg
					}
				: null,
		[factor, globalTransformData]
	);
	const interactionMode = useInteractionMode(artboard);
	const showHandles = React.useMemo(
		() =>
			interactionMode !== 'Translating' &&
			interactionMode !== 'Rotating' &&
			interactionMode !== 'Dragging',
		[interactionMode]
	);
	const handlePointerEvents = React.useMemo(
		() => (interactionMode === 'Resizing' || interactionMode === 'Rotating' ? 'none' : 'auto'),
		[interactionMode]
	);

	// =========================================================================
	// Callbacks
	// =========================================================================

	const handleOnResizeHandlePointerEvent = React.useCallback(
		(side: EHandleSide, eventType: 'Up' | 'Down', event: React.PointerEvent<SVGGElement>) => {
			event.stopPropagation();
			if (sizeData == null || globalTransformData == null) {
				return;
			}
			const { size } = sizeData;
			const { translation, rotationDeg } = globalTransformData;

			if (event.button === 0) {
				switch (eventType) {
					case 'Up':
						onResizeHandlePointerUp(artboard.builder.pointerEventToArbPoint(event));
						break;
					case 'Down':
						onResizeHandlePointerDown(
							side,
							{
								position: translation,
								size
							},
							rotationDeg
						);
						break;
				}
			}
		},
		[
			sizeData,
			globalTransformData,
			artboard.builder,
			onResizeHandlePointerUp,
			onResizeHandlePointerDown
		]
	);

	const handleOnRotateHandlePointerEvent = React.useCallback(
		(side: EHandleSide, eventType: 'Up' | 'Down', event: React.PointerEvent<SVGGElement>) => {
			event.stopPropagation();
			if (globalTransformData == null) {
				return;
			}
			const { rotationDeg } = globalTransformData;

			if (event.button === 0) {
				switch (eventType) {
					case 'Up':
						onRotateHandlePointerUp(artboard.builder.pointerEventToArbPoint(event));
						break;
					case 'Down':
						onRotateHandlePointerDown(side, rotationDeg);
						break;
				}
			}
		},
		[globalTransformData, artboard.builder, onRotateHandlePointerUp, onRotateHandlePointerDown]
	);

	// =========================================================================
	// UI
	// =========================================================================

	if (
		sizeData == null ||
		factoredSizeData == null ||
		globalTransformData == null ||
		factoredTransformData == null
	) {
		return null;
	}

	const { size: factoredSize } = factoredSizeData;
	const { size } = sizeData;
	const { rotationDeg: factoredRotation, translation: factoredTranslation } = factoredTransformData;

	return (
		<g
			style={{
				transform: `translate(${(factoredTranslation[0] - artboard.viewport.physicalPosition[0] * factor[0]).toString()}px, ${(
					factoredTranslation[1] -
					artboard.viewport.physicalPosition[1] * factor[1]
				).toString()}px) rotate(${factoredRotation.toString()}deg)`
			}}
		>
			{/* Selection Border */}
			<rect
				className="pointer-events-none fill-transparent stroke-blue-400 stroke-1"
				height={factoredSize[1]}
				width={factoredSize[0]}
				x={0}
				y={0}
			/>

			{showHandles ? (
				<g id="handles">
					{/* Resize Edge Handles*/}
					<ResizeEdgeHandle
						cursor={CURSOR.resize(factoredRotation)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Top, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Top, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="top"
					/>
					<ResizeEdgeHandle
						cursor={CURSOR.resize(factoredRotation + 90)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Left, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Left, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="left"
					/>
					<ResizeEdgeHandle
						cursor={CURSOR.resize(factoredRotation)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Bottom, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Bottom, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="bottom"
					/>
					<ResizeEdgeHandle
						cursor={CURSOR.resize(factoredRotation + 90)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Right, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Right, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="right"
					/>

					{/* Resize Corner Handles*/}
					<ResizeCornerHandle
						cursor={CURSOR.resize(factoredRotation + 135)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Top + EHandleSide.Left, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Top + EHandleSide.Left, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="topLeft"
					/>
					<ResizeCornerHandle
						cursor={CURSOR.resize(factoredRotation - 135)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Top + EHandleSide.Right, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Top + EHandleSide.Right, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="topRight"
					/>
					<ResizeCornerHandle
						cursor={CURSOR.resize(factoredRotation + 135)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Right, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Right, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="bottomRight"
					/>
					<ResizeCornerHandle
						cursor={CURSOR.resize(factoredRotation - 135)}
						onPointerDown={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Left, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnResizeHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Left, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="bottomLeft"
					/>

					{/* Rotate Corner Handles*/}
					<RotateCornerHandle
						cursor={CURSOR.rotate(factoredRotation)}
						offset={15}
						onPointerDown={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Top + EHandleSide.Left, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Top + EHandleSide.Left, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="topLeft"
					/>
					<RotateCornerHandle
						cursor={CURSOR.rotate(factoredRotation + 90)}
						offset={15}
						onPointerDown={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Top + EHandleSide.Right, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Top + EHandleSide.Right, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="topRight"
					/>
					<RotateCornerHandle
						cursor={CURSOR.rotate(factoredRotation + 180)}
						offset={15}
						onPointerDown={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Right, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Right, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="bottomRight"
					/>
					<RotateCornerHandle
						cursor={CURSOR.rotate(factoredRotation - 90)}
						offset={15}
						onPointerDown={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Left, 'Down', e);
						}}
						onPointerUp={(e) => {
							handleOnRotateHandlePointerEvent(EHandleSide.Bottom + EHandleSide.Left, 'Up', e);
						}}
						parentSize={factoredSize}
						pointerEvents={handlePointerEvents}
						position="bottomLeft"
					/>
				</g>
			) : null}

			{/* Dimension Indicator */}
			{showHandles ? (
				<foreignObject
					className="pointer-events-none overflow-visible"
					height="40"
					width={factoredSize[0]}
					x={0}
					y={factoredSize[1]}
				>
					<div className="flex h-full items-center justify-center">
						<div
							className="whitespace-nowrap rounded-sm bg-blue-500 px-2 py-1 text-center text-xs text-white"
							style={{ minWidth: 'min-content' }}
						>
							{size[0].toFixed(0)} x {size[1].toFixed(0)}
						</div>
					</div>
				</foreignObject>
			) : null}
		</g>
	);
};

interface TProps {
	entity: ARB.Entity;
	artboard: Artboard;
	onResizeHandlePointerDown: (
		corner: EHandleSide,
		initialBounds: ARB.XYWH,
		rotationDeg: number
	) => void;
	onResizeHandlePointerUp: (position: ARB.Vec2) => void;
	onRotateHandlePointerDown: (side: EHandleSide, rotationDeg: number) => void;
	onRotateHandlePointerUp: (position: ARB.Vec2) => void;
}

export enum EHandleSide {
	Top = 1,
	Bottom = 2,
	Left = 4,
	Right = 8
}
