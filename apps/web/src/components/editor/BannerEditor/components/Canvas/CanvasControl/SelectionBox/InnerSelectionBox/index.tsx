import React from 'react';
import { radiansToDegrees } from '@dyn/dtif';
import type { COMP, Composition, SVGRender } from '@dyn/svg-composition';

import { useInteractionMode, useMatrixTransform, useWatchEntity } from '../../../../../hooks';
import { getHandleMetaData as getHandlePositions, type EHandleSide } from './controller';
import { Handle } from './Handle';

export const InnerSelectionBox: React.FC<TProps> = React.memo((props) => {
	const {
		entity,
		composition,
		showHandles,
		onResizeHandlePointerDown,
		onResizeHandlePointerUp,
		onRotateHandlePointerDown,
		onRotateHandlePointerUp
	} = props;
	const {
		Dimension: { width = undefined, height = undefined } = {},
		RelativeTransform: { relativeTransform = undefined } = {}
	} = useWatchEntity(composition, entity, ['Dimension', 'RelativeTransform']);
	const viewWidthFactor = React.useMemo(
		() => composition.width / composition.viewBox.width,
		[composition.width, composition.viewBox.width]
	);
	const viewHeightFactor = React.useMemo(
		() => composition.height / composition.viewBox.height,
		[composition.height, composition.viewBox.height]
	);
	const { tx: x, ty: y, rotationInRadians } = useMatrixTransform(relativeTransform);
	const interactionMode = useInteractionMode(composition);
	const handlePositions = React.useMemo(() => {
		if (width == null || height == null) {
			return null;
		}
		return getHandlePositions(width * viewWidthFactor, height * viewHeightFactor);
	}, [width, height, viewWidthFactor, viewHeightFactor]);

	if (
		width == null ||
		height == null ||
		x == null ||
		y == null ||
		rotationInRadians == null ||
		handlePositions == null
	) {
		return null;
	}
	const rotationInDegrees = radiansToDegrees(rotationInRadians);

	return (
		<g
			style={{
				transform: `translate(${x * viewWidthFactor}px, ${
					y * viewHeightFactor
				}px) rotate(${-rotationInDegrees}deg)`
			}}
		>
			{/* Border */}
			<rect
				className="pointer-events-none fill-transparent stroke-blue-400 stroke-1"
				height={height * viewHeightFactor}
				width={width * viewWidthFactor}
				x={0}
				y={0}
			/>

			{/* Dimension Indicator */}
			{showHandles ? (
				<foreignObject
					className="overflow-visible"
					height="40"
					width={width * viewWidthFactor}
					x={0}
					y={height * viewHeightFactor}
				>
					<div className="flex h-full items-center justify-center">
						<div
							className="whitespace-nowrap rounded-sm bg-blue-500 px-2 py-1 text-center text-xs text-white"
							style={{ minWidth: 'min-content' }}
						>
							{width.toFixed(0)} x {height.toFixed(0)}
						</div>
					</div>
				</foreignObject>
			) : null}

			{showHandles
				? handlePositions.map((handle, index) => {
						return (
							<Handle
								key={index}
								pointerEvents={
									interactionMode.type === 'Resizing' || interactionMode.type === 'Rotating'
										? 'none'
										: 'auto'
								}
								position={handle.position}
								resizeHandle={{
									width: handle.resizeHandle.width,
									height: handle.resizeHandle.height,
									pointerAreaOffset: handle.resizeHandle.pointerAreaOffset,
									cursor: handle.resizeHandle.cursor.toString(rotationInDegrees),
									onPointerDown: (e) => {
										e.stopPropagation();
										onResizeHandlePointerDown(
											handle.corner,
											{
												position: [x, y],
												height,
												width
											},
											rotationInRadians
										);
									},
									onPointerUp: (e) => {
										e.stopPropagation();
										// TODO: Can this be done more typesafe?
										onResizeHandlePointerUp(
											(composition.renderer[0] as SVGRender).pointerEventToCompositionPoint(
												e as unknown as PointerEvent
											)
										);
									}
								}}
								rotateHandle={
									handle.rotateHandle
										? {
												width: handle.rotateHandle.width,
												height: handle.rotateHandle.height,
												cursor: handle.rotateHandle.cursor.toString(rotationInDegrees),
												offset: handle.rotateHandle.offset,
												onPointerDown: (e) => {
													e.stopPropagation();
													onRotateHandlePointerDown(handle.corner, rotationInRadians);
												},
												onPointerUp: (e) => {
													e.stopPropagation();
													// TODO: Can this be done more typesafe?
													onRotateHandlePointerUp(
														(composition.renderer[0] as SVGRender).pointerEventToCompositionPoint(
															e as unknown as PointerEvent
														)
													);
												}
										  }
										: false
								}
							/>
						);
				  })
				: null}
		</g>
	);
});
InnerSelectionBox.displayName = 'InnerSelectionBox';

interface TProps {
	entity: COMP.Entity;
	composition: Composition;
	showHandles: boolean;
	onResizeHandlePointerDown: (
		corner: EHandleSide,
		initialBounds: COMP.XYWH,
		rotationInRadians: number
	) => void;
	onResizeHandlePointerUp: (position: COMP.Vec2) => void;
	onRotateHandlePointerDown: (corner: EHandleSide, rotationInRadians: number) => void;
	onRotateHandlePointerUp: (position: COMP.Vec2) => void;
}
