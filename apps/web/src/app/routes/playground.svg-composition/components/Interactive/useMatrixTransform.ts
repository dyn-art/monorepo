import React from 'react';
import { extract2DTransformMatrixData, Mat3, T2DMatrixData } from '@dyn/svg-composition';

export function useMatrixTransform(transform: Mat3): T2DMatrixData;
export function useMatrixTransform(transform?: Mat3): Partial<T2DMatrixData>;

export function useMatrixTransform(transform?: Mat3): T2DMatrixData | Partial<T2DMatrixData> {
	return React.useMemo(() => {
		return transform ? extract2DTransformMatrixData(transform) : {};
	}, [transform]);
}
