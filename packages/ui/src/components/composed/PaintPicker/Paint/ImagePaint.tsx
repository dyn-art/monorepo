'use client';

import type { TVec2 } from '@ibg/utils';
import React from 'react';

import type { TImagePaint } from '../types';

export const ImagePaint: React.FC<TProps> = (props) => {
	const { paint, size, ...other } = props;
	const { content, opacity } = paint;
	const [imageUrl, setImageUrl] = React.useState<string | null>(null);

	React.useEffect(() => {
		if (content != null) {
			const blob = new Blob([new Uint8Array(content)], { type: 'image/png' });
			const url = URL.createObjectURL(blob);
			setImageUrl(url);

			return () => {
				URL.revokeObjectURL(url);
			};
		}
	}, [content]);

	return (
		<div
			{...other}
			style={{
				...other.style,
				backgroundImage: imageUrl
					? `url(${imageUrl})`
					: `linear-gradient(45deg, #ccc 25%, transparent 25%, transparent 75%, #ccc 75%, #ccc), linear-gradient(45deg, #ccc 25%, transparent 25%, transparent 75%, #ccc 75%, #ccc)`,
				backgroundSize: imageUrl ? 'cover' : '8px 8px',
				backgroundPosition: imageUrl ? 'center' : '0 0, 4px 4px',
				opacity,
				width: size[0],
				height: size[1]
			}}
		/>
	);
};

interface TProps extends React.HTMLAttributes<HTMLDivElement> {
	paint: TImagePaint;
	size: TVec2;
}
