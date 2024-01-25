'use client';

import React from 'react';
import type { COMP, Composition } from '@dyn/svg-composition';
import { Button, CircleIcon, Component2Icon, Skeleton, SquareIcon } from '@dyn/ui';

import { Canvas, CompositionControl, type TCanvasProps } from './components';

export const BannerEditor: React.FC<TBannerEditorProps> = (props) => {
	const { width, height, dtif: defaultDTIF } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const [dtif, setDTIF] = React.useState<COMP.DTIFComposition | null>(null);

	React.useEffect(() => {
		(async () => {
			const text = await navigator.clipboard.readText();
			try {
				const maybeDTIF = JSON.parse(text);
				if ('version' in maybeDTIF) {
					setDTIF(maybeDTIF as COMP.DTIFComposition);
				} else {
					throw new Error('Invalid DTIF');
				}
			} catch (e) {
				setDTIF(defaultDTIF);
			}
		})().catch(() => {
			// do nothing
		});
	}, []);

	return (
		<div className="flex flex-col items-center justify-center">
			<div style={{ width: width + 4, height: height + 4 }}>
				{dtif != null ? (
					<Canvas dtif={dtif} height={height} onLoadedComposition={setComposition} width={width} />
				) : (
					<Skeleton className="h-full w-full" />
				)}
			</div>
			<div className="flex w-full flex-row items-center justify-between ">
				<div>TODO</div>
				<div>
					<Button
						onClick={() => {
							console.log({ composition });
							if (composition != null) {
								composition.spawnRectangle({
									x: 10,
									y: 10,
									width: 100,
									height: 100,
									color: [0, 0, 0]
								});
								composition.update();
							}
						}}
						size="icon"
						variant="outline"
					>
						<SquareIcon className="h-4 w-4" />
					</Button>
					<Button size="icon" variant="outline">
						<CircleIcon className="h-4 w-4" />
					</Button>
					<Button size="icon" variant="outline">
						<Component2Icon
							className="h-4 w-4"
							onClick={() => {
								composition?.updateViewBox({
									minX: 0,
									minY: 0,
									width: composition.width,
									height: composition.height
								});
							}}
						/>
					</Button>
				</div>
				<div className="flex flex-row gap-2">
					<Button variant="outline">Preview</Button>
					<Button
						onClick={() => {
							console.log(composition?.toString());
						}}
					>
						Export
					</Button>
				</div>
			</div>
			{composition != null && <CompositionControl composition={composition} />}
		</div>
	);
};

export type TBannerEditorProps = {
	// TODO:
} & Omit<TCanvasProps, 'onLoadedComposition'>;
