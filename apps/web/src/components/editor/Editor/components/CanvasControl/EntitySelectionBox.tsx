import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';

import { useEntity, useInteractionMode, useViewportFactor, type TComponent } from '../../hooks';
import { ResizeCornerHandle } from './ResizeCornerHandle';
import { ResizeEdgeHandle } from './ResizeEdgeHandle';
import { RotateCornerHandle } from './RotateCornerHandle';

export const EntitySelectionBox: React.FC<TProps> = (props) => {
	const {
		composition,
		entity,
		onResizeHandlePointerDown,
		onResizeHandlePointerUp,
		onRotateHandlePointerDown,
		onRotateHandlePointerUp
	} = props;
	const { Size: sizeData, GlobalTransform: globalTransformData } = useEntity(composition, entity, [
		'Size',
		'GlobalTransform'
	]);
	const factor = useViewportFactor(composition);
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
	const interactionMode = useInteractionMode(composition);
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
			if (sizeData == null || globalTransformData == null || composition.renderer == null) {
				return;
			}
			const { size } = sizeData;
			const { translation, rotationDeg } = globalTransformData;

			if (event.button === 0) {
				switch (eventType) {
					case 'Up':
						onResizeHandlePointerUp(composition.renderer.pointerEventToCompPoint(event));
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
			composition.renderer,
			onResizeHandlePointerUp,
			onResizeHandlePointerDown
		]
	);

	const handleOnRotateHandlePointerEvent = React.useCallback(
		(side: EHandleSide, eventType: 'Up' | 'Down', event: React.PointerEvent<SVGGElement>) => {
			event.stopPropagation();
			if (globalTransformData == null || composition.renderer == null) {
				return;
			}
			const { rotationDeg } = globalTransformData;

			if (event.button === 0) {
				switch (eventType) {
					case 'Up':
						onRotateHandlePointerUp(composition.renderer.pointerEventToCompPoint(event));
						break;
					case 'Down':
						onRotateHandlePointerDown(side, rotationDeg);
						break;
				}
			}
		},
		[globalTransformData, composition.renderer, onRotateHandlePointerUp, onRotateHandlePointerDown]
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
				transform: `translate(${factoredTranslation[0] - composition.viewport.physicalPosition[0] * factor[0]}px, ${
					factoredTranslation[1] - composition.viewport.physicalPosition[1] * factor[0]
				}px) rotate(${factoredRotation}deg)`
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
					<ResizeCornerHandle
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

					{/* Rotate Corner Handles*/}
					<RotateCornerHandle
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
					<RotateCornerHandle
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
	entity: COMP.Entity;
	composition: Composition;
	onResizeHandlePointerDown: (
		corner: EHandleSide,
		initialBounds: COMP.XYWH,
		rotationDeg: number
	) => void;
	onResizeHandlePointerUp: (position: COMP.Vec2) => void;
	onRotateHandlePointerDown: (side: EHandleSide, rotationDeg: number) => void;
	onRotateHandlePointerUp: (position: COMP.Vec2) => void;
}

export enum EHandleSide {
	Top = 1,
	Bottom = 2,
	Left = 4,
	Right = 8
}
