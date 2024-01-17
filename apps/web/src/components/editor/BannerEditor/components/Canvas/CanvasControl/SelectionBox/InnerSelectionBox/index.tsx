import React from 'react';
import { radiansToDegrees } from '@dyn/dtif';
import {
	type Composition,
	type Entity,
	type SVGRenderer,
	type Vec2,
	type XYWH
} from '@dyn/svg-composition';

import { useInteractionMode } from '../../../../../hooks/use-interaction-mode';
import { useMatrixTransform } from '../../../../../hooks/use-matrix-transform';
import { useWatchEntity } from '../../../../../hooks/use-watch-entity';
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
	const { tx: x, ty: y, rotationInRadians } = useMatrixTransform(relativeTransform);
	const interactionMode = useInteractionMode(composition);
	const handlePositions = React.useMemo(() => {
		if (width == null || height == null) {
			return null;
		}
		return getHandlePositions(width, height);
	}, [width, height]);

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
		<g style={{ transform: `translate(${x}px, ${y}px) rotate(${-rotationInDegrees}deg)` }}>
			{/* Border */}
			<rect
				className="pointer-events-none fill-transparent stroke-blue-400 stroke-1"
				height={height}
				width={width}
				x={0}
				y={0}
			/>

			{/* Dimension Indicator */}
			{showHandles ? (
				<foreignObject className="overflow-visible" height="40" width={width} x={0} y={height}>
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
											(composition.renderer[0] as SVGRenderer).pointerEventToCompositionPoint(
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
														(composition.renderer[0] as SVGRenderer).pointerEventToCompositionPoint(
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
	entity: Entity;
	composition: Composition;
	showHandles: boolean;
	onResizeHandlePointerDown: (
		corner: EHandleSide,
		initialBounds: XYWH,
		rotationInRadians: number
	) => void;
	onResizeHandlePointerUp: (position: Vec2) => void;
	onRotateHandlePointerDown: (corner: EHandleSide, rotationInRadians: number) => void;
	onRotateHandlePointerUp: (position: Vec2) => void;
}
