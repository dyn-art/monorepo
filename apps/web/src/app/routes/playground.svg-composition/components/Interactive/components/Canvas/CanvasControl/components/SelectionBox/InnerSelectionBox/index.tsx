import React from 'react';
import {
	Composition,
	Entity,
	radiansToDegrees,
	SVGRenderer,
	Vec2,
	XYWH
} from '@dyn/svg-composition';

import { useInteractionMode } from '../../../../../../hooks/useInteractionMode';
import { useMatrixTransform } from '../../../../../../hooks/useMatrixTransform';
import { useWatchEntity } from '../../../../../../hooks/useWatchEntity';
import { EHandleSide, getHandleMetaData as getHandlePositions } from './controller';
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
				className={'pointer-events-none fill-transparent stroke-blue-400 stroke-1'}
				x={0}
				y={0}
				width={width}
				height={height}
			/>

			{/* Dimension Indicator */}
			{showHandles && (
				<foreignObject x={0} y={height} width={width} height="40" className="overflow-visible">
					<div className="flex h-full items-center justify-center">
						<div
							className="whitespace-nowrap rounded-sm bg-blue-500 px-2 py-1 text-center text-xs text-white"
							style={{ minWidth: 'min-content' }}
						>
							{width} x {height}
						</div>
					</div>
				</foreignObject>
			)}

			{showHandles &&
				handlePositions.map((handle, index) => {
					return (
						<Handle
							key={index}
							position={handle.position}
							pointerEvents={
								interactionMode.type === 'Resizing' || interactionMode.type === 'Rotating'
									? 'none'
									: 'auto'
							}
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
				})}
		</g>
	);
});

type TProps = {
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
};
