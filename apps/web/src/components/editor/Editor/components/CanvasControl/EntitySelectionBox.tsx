import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';

import { useEntity, useViewportFactor, type TComponent } from '../../hooks';
import { ResizeCornerHandle } from './ResizeCornerHandle';
import { ResizeEdgeHandle } from './ResizeEdgeHandle';
import { RotateCornerHandle } from './RotateCornerHandle';

export const EntitySelectionBox: React.FC<TProps> = (props) => {
	const { composition, entity, showHandles = true } = props;
	const { Size: sizeData, Transform: transformData } = useEntity(composition, entity, [
		'Size',
		'Transform'
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
			transformData != null
				? {
						translation: [
							transformData.translation[0] * factor[0],
							transformData.translation[1] * factor[1]
						],
						rotationDeg: transformData.rotationDeg
					}
				: null,
		[factor, transformData]
	);

	if (
		sizeData == null ||
		factoredSizeData == null ||
		transformData == null ||
		factoredTransformData == null
	) {
		return;
	}
	const { size: factoredSize } = factoredSizeData;
	const { size } = sizeData;
	const { rotationDeg: rotation, translation: factoredTranslation } = factoredTransformData;

	// =========================================================================
	// UI
	// =========================================================================

	return (
		<g
			style={{
				transform: `translate(${factoredTranslation[0] - composition.viewport.physicalPosition[0]}px, ${
					factoredTranslation[1] - composition.viewport.physicalPosition[1]
				}px) rotate(${-rotation}deg)`
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
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="top"
					/>
					<ResizeEdgeHandle
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="left"
					/>
					<ResizeEdgeHandle
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="bottom"
					/>
					<ResizeEdgeHandle
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="right"
					/>

					{/* Resize Corner Handles*/}
					<ResizeCornerHandle
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="topLeft"
					/>
					<ResizeCornerHandle
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="topRight"
					/>
					<ResizeCornerHandle
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="bottomLeft"
					/>
					<ResizeCornerHandle
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="bottomRight"
					/>

					{/* Rotate Corner Handles*/}
					<RotateCornerHandle
						offset={15}
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="topLeft"
					/>
					<RotateCornerHandle
						offset={15}
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="topRight"
					/>
					<RotateCornerHandle
						offset={15}
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="bottomLeft"
					/>
					<RotateCornerHandle
						offset={15}
						onPointerDown={() => {}}
						onPointerUp={() => {}}
						parentSize={factoredSize}
						position="bottomRight"
					/>
				</g>
			) : null}

			{/* Dimension Indicator */}
			{showHandles ? (
				<foreignObject
					className="overflow-visible"
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
	showHandles?: boolean;
}
