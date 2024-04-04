import React from 'react';

import { useSizeCallback, type TSize } from './use-size-callback';

export function useSize<T extends HTMLElement = HTMLElement>(
	ref: React.RefObject<T>
): TSize | null {
	const [size, setSize] = React.useState<TSize | null>(null);
	useSizeCallback(ref, setSize, []);
	return size;
}
