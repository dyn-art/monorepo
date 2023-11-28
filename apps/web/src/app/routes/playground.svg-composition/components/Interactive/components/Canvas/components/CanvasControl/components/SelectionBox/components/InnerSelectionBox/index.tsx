import React from 'react';
import { Composition, Entity, SVGRenderer, Vec2, XYWH } from '@dyn/svg-composition';

import { Handle } from '..';
import { useInteractionMode } from '../../../../../../../../useInteractionMode';
import { useMatrixTransform } from '../../../../../../../../useMatrixTransform';
import { useWatchEntity } from '../../../../../../../../useWatchEntity';
import { EHandleSide, getHandleMetaData as getHandlePositions } from './controller';

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
	const { tx: x, ty: y, rotation } = useMatrixTransform(relativeTransform);
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
		rotation == null ||
		handlePositions == null
	) {
		return null;
	}

	return (
		<g style={{ transform: `translate(${x}px, ${y}px) rotate(${-rotation}deg)` }}>
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
							position={[handle.x, handle.y]}
							pointerEvents={interactionMode.type === 'Resizing' ? 'none' : 'auto'}
							resizeHandle={{
								width: handle.width,
								height: handle.height,
								cursor: handle.resizeHandle.cursor.toString(rotation),
								onPointerDown: (e) => {
									e.stopPropagation();
									onResizeHandlePointerDown(
										handle.corner,
										{
											position: [x, y],
											height,
											width
										},
										rotation
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
							rotateHandle={{
								size: 20,
								cursor: handle.resizeHandle.cursor.toString(rotation),
								offset: 5,
								onPointerDown: (e) => {
									e.stopPropagation();
									onRotateHandlePointerDown(handle.corner);
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
							}}
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
	onResizeHandlePointerDown: (corner: EHandleSide, initialBounds: XYWH, rotation: number) => void;
	onResizeHandlePointerUp: (position: Vec2) => void;
	onRotateHandlePointerDown: (corner: EHandleSide) => void;
	onRotateHandlePointerUp: (position: Vec2) => void;
};
