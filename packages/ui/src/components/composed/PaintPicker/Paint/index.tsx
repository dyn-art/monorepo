import React from 'react';
import type { TVec2 } from '@dyn/utils';

import type { TPaint } from '../types';
import { GradientPaint } from './GradientPaint';
import { ImagePaint } from './ImagePaint';
import { SolidPaint } from './SolidPaint';

export * from './GradientPaint';
export * from './SolidPaint';

export const Paint: React.FC<TProps> = (props) => {
	const { paint, size, className } = props;

	switch (paint.type) {
		case 'Solid':
			return <SolidPaint className={className} paint={paint} size={size} />;
		case 'Gradient':
			return <GradientPaint className={className} paint={paint} size={size} />;
		case 'Image':
			return <ImagePaint className={className} paint={paint} size={size} />;
	}
};

interface TProps {
	paint: TPaint;
	size: TVec2;
	className?: string;
}
