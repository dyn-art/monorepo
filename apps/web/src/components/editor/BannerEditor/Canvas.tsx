'use client';

import dynamic from 'next/dynamic';
import React from 'react';
import type { TComposition } from '@dyn/dtif';
import { createSVGComposition, type Composition } from '@dyn/svg-composition';
import { Skeleton } from '@dyn/ui';

export const Canvas = dynamic(
	async () => {
		const { initWasm } = await import('@dyn/svg-composition');

		await initWasm();

		// eslint-disable-next-line react/display-name, func-names -- j
		return function (props: TCanvasProps) {
			const { width, height, dtif } = props;
			const svgContainerRef = React.useRef<HTMLDivElement>(null);
			const [composition, setComposition] = React.useState<Composition | null>(null);

			React.useEffect(() => {
				if (svgContainerRef.current != null && composition == null) {
					const newComposition = createSVGComposition({
						width,
						height,
						renderer: {
							domElement: svgContainerRef.current
						},
						dtif
					});
					setComposition(newComposition);
					newComposition.update();
				}

				return () => {
					if (composition != null) {
						composition.unmount();
					}
				};
			}, [composition, width, height, dtif]);

			return <div ref={svgContainerRef} />;
		};
	},
	{
		loading: () => <Skeleton className="h-full w-full" />,
		ssr: false
	}
);

export interface TCanvasProps {
	width: number;
	height: number;
	dtif?: TComposition;
}
