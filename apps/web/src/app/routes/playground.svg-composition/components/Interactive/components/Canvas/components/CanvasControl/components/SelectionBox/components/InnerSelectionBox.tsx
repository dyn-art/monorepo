import React from 'react';
import { Composition, Entity, SVGRenderer, Vec2, XYWH } from '@dyn/svg-composition';

import { useInteractionMode } from '../../../../../../../useInteractionMode';
import { useMatrixTransform } from '../../../../../../../useMatrixTransform';
import { useWatchEntity } from '../../../../../../../useWatchEntity';
import { Handle } from './Handle';

const MIDDLE_HANDLE_WIDTH = 8; // px
const MIDDLE_HANDLE_HEIGHT = 16; // px
const CORNER_HANDLE_SIZE = 8; // px

export const InnerSelectionBox: React.FC<TProps> = React.memo((props) => {
	const { entity, composition, showHandles, onResizeHandlePointerDown, onResizeHandlePointerUp } =
		props;
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

		return [
			{
				x: -CORNER_HANDLE_SIZE / 2,
				y: -CORNER_HANDLE_SIZE / 2,
				cursor: 'nwse-resize',
				resizeHandle: EHandleSide.Top + EHandleSide.Left
			},
			{
				x: width / 2 - MIDDLE_HANDLE_HEIGHT / 2,
				y: -MIDDLE_HANDLE_WIDTH / 2,
				cursor: 'ns-resize',
				resizeHandle: EHandleSide.Top
			},
			{
				x: width - CORNER_HANDLE_SIZE / 2,
				y: -CORNER_HANDLE_SIZE / 2,
				cursor: 'nesw-resize',
				resizeHandle: EHandleSide.Top + EHandleSide.Right
			},
			{
				x: width - MIDDLE_HANDLE_WIDTH / 2,
				y: height / 2 - MIDDLE_HANDLE_HEIGHT / 2,
				cursor: 'ew-resize',
				resizeHandle: EHandleSide.Right
			},
			{
				x: width - CORNER_HANDLE_SIZE / 2,
				y: height - CORNER_HANDLE_SIZE / 2,
				cursor: 'nwse-resize',
				resizeHandle: EHandleSide.Bottom + EHandleSide.Right
			},
			{
				x: width / 2 - MIDDLE_HANDLE_HEIGHT / 2,
				y: height - MIDDLE_HANDLE_WIDTH / 2,
				cursor: 'ns-resize',
				resizeHandle: EHandleSide.Bottom
			},
			{
				x: -CORNER_HANDLE_SIZE / 2,
				y: height - CORNER_HANDLE_SIZE / 2,
				cursor: 'nesw-resize',
				resizeHandle: EHandleSide.Bottom + EHandleSide.Left
			},
			{
				x: -MIDDLE_HANDLE_WIDTH / 2,
				y: height / 2 - MIDDLE_HANDLE_HEIGHT / 2,
				cursor: 'ew-resize',
				resizeHandle: EHandleSide.Left
			}
		];
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

			{/* Handles */}
			{showHandles &&
				handlePositions.map((pos, index) => {
					let handleRotation = 0;
					let handleWidth = CORNER_HANDLE_SIZE;
					let handleHeight = CORNER_HANDLE_SIZE;
					switch (pos.resizeHandle) {
						case EHandleSide.Top + EHandleSide.Left:
							handleRotation = -135;
							break;
						case EHandleSide.Top:
							handleRotation = 90;
							handleWidth = MIDDLE_HANDLE_HEIGHT;
							handleHeight = MIDDLE_HANDLE_WIDTH;
							break;
						case EHandleSide.Top + EHandleSide.Right:
							handleRotation = 135;
							break;
						case EHandleSide.Right:
							handleRotation = 0;
							handleWidth = MIDDLE_HANDLE_WIDTH;
							handleHeight = MIDDLE_HANDLE_HEIGHT;
							break;
						case EHandleSide.Bottom + EHandleSide.Right:
							handleRotation = 45;
							break;
						case EHandleSide.Bottom:
							handleRotation = 90;
							handleWidth = MIDDLE_HANDLE_HEIGHT;
							handleHeight = MIDDLE_HANDLE_WIDTH;
							break;
						case EHandleSide.Bottom + EHandleSide.Left:
							handleRotation = -45;
							break;
						case EHandleSide.Left:
							handleRotation = 0;
							handleWidth = MIDDLE_HANDLE_WIDTH;
							handleHeight = MIDDLE_HANDLE_HEIGHT;
							break;
					}

					// Include the rotation of the object itself
					handleRotation = handleRotation - rotation;

					const cursor = `url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' width='20px' height='20px' %3E%3Cg transform='rotate(${handleRotation} 8,8)'%3E%3C%70%61%74%68%20%66%69%6C%6C%3D%27%23%66%66%66%27%20%64%3D%27%4D%32%2E%36%20%35%2E%36%4C%30%20%38%2E%33%20%32%2E%36%20%31%31%6c%31%2E%32%2D%31%2E%32%2D%2E%35%2D%2E%35%68%39%2E%34%6c%2D%2E%35%2E%35%20%31%2E%32%20%31%2E%32%4C%31%36%20%38%2E%33%6c%2D%32%2E%36%2D%32%2E%37%2D%31%2E%32%20%31%2E%32%2E%35%2E%35%48%33%2E%33%6c%2E%35%2D%2E%35%2D%31%2E%32%2D%31%2E%32%7A%27%2F%3E%3C%70%61%74%68%20%66%69%6C%6C%3D%27%23%32%33%31%66%32%30%27%20%64%3D%27%4D%35%2E%31%20%32%37%39%68%2D%34%76%31%68%35%76%2D%35%68%2D%31%7A%6D%35%20%30%76%35%68%2D%35%76%31%68%35%76%35%68%31%76%2D%35%68%35%76%2D%31%68%2D%35%76%2D%35%7A%27%2F%3E%3C%70%61%74%68%20%66%69%6C%6C%3D%27%23%66%66%66%27%20%64%3D%27%4D%2E%36%20%32%37%38%2E%35%68%34%76%2D%34%68%32%76%36%68%2D%36%7A%6D%34%2E%35%2E%35%68%2D%34%76%31%68%35%76%2D%35%68%2D%31%7A%6D%34%2E%35%2D%2E%35%68%32%76%35%68%35%76%32%68%2D%35%76%35%68%2D%32%76%2D%35%68%2D%35%76%2D%32%68%35%7A%6D%2E%35%20%35%2E%35%68%2D%35%76%31%68%35%76%35%68%31%76%2D%35%68%35%76%2D%31%68%2D%35%76%2D%35%68%2D%31%7A%27%2F%3E%3C%70%61%74%68%20%66%69%6C%6C%3D%27%23%30%30%30%27%20%64%3D%27%4D%32%2E%36%20%36%2E%33%6c%2D%32%20%32%20%32%20%32%20%2E%36%2D%2E%35%2D%31%2D%31%48%31%34%6c%2D%31%20%31%20%2E%35%2E%35%20%32%2D%32%2D%32%2D%32%2D%2E%35%2E%35%20%31%20%31%48%32%2E%31%6c%31%2D%31%2D%2E%35%2D%2E%35%7A%27%2F%3E%3C/g%3E%3C/svg%3E") 12 12, auto`;

					return (
						<Handle
							key={index}
							x={pos.x}
							y={pos.y}
							width={handleWidth}
							height={handleHeight}
							cursor={cursor}
							pointerEvents={interactionMode.type === 'Resizing' ? 'none' : 'auto'}
							onPointerDown={(e) => {
								e.stopPropagation();
								onResizeHandlePointerDown(pos.resizeHandle, {
									position: [x, y],
									height,
									width
								});
							}}
							onPointerUp={(e) => {
								e.stopPropagation();
								// TODO: Can this be done more typesafe?
								onResizeHandlePointerUp(
									(composition.renderer[0] as SVGRenderer).pointerEventToCompositionPoint(
										e as unknown as PointerEvent
									)
								);
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
	onResizeHandlePointerDown: (corner: EHandleSide, initialBounds: XYWH) => void;
	onResizeHandlePointerUp: (position: Vec2) => void;
};

export enum EHandleSide {
	Top = 1,
	Bottom = 2,
	Left = 4,
	Right = 8
}
