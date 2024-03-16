import React from 'react';
import { extract2DTransformMatrixData, type T2DMatrixData } from '@dyn/dtif';
import type { COMP } from '@dyn/svg-composition';

export function useMatrixTransform(transform: COMP.Mat3): T2DMatrixData;
export function useMatrixTransform(transform?: COMP.Mat3): Partial<T2DMatrixData>;

export function useMatrixTransform(transform?: COMP.Mat3): Partial<T2DMatrixData> {
	return React.useMemo(() => {
		return transform ? extract2DTransformMatrixData(transform) : {};
	}, [transform]);
}
