'use client';

import React from 'react';
import type { Composition } from '@dyn/svg-comp';
import { Button, Skeleton } from '@dyn/ui';

import { useDtifFromClipboard } from '../hooks';
import { Canvas, type TCanvasProps } from './Canvas';

export const InnerEditor: React.FC<TInnerEditorProps> = (props) => {
	const { width, height, dtif: defaultDtif } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const { isLoading, data: dtif } = useDtifFromClipboard(defaultDtif);

	if (isLoading || dtif == null) {
		return <Skeleton className="h-full w-full" />;
	}

	return (
		<div className="flex flex-col items-center justify-center">
			<Canvas dtif={dtif} height={height} onLoadedComposition={setComposition} width={width} />
			<div className="flex w-full flex-row items-center justify-between ">
				<Button
					onClick={() => {
						console.log(composition?.toString());
					}}
				>
					To String
				</Button>
				<Button
					onClick={() => {
						if (composition != null) {
							for (const selectedEntity of composition.selectedEntities) {
								composition.emitInputEvent({
									type: 'Composition',
									event: { type: 'EntitySetRotation', entity: selectedEntity, rotationDeg: 45 }
								});
							}
							composition.update();
						}
					}}
				>
					Rotate Selected
				</Button>
			</div>
		</div>
	);
};

export type TInnerEditorProps = {
	// TODO:
} & Omit<TCanvasProps, 'onLoadedComposition'>;
