import React from 'react';
import type { TMdtifComposition } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';
import { Badge, Skeleton, useSizeCallback } from '@dyn/ui';
import { usePreparedDtif } from '@/hooks';

import { ModificationInput, Viewport } from './components';

export const FieldBasedEditor: React.FC<TFieldBasedEditorProps> = (props) => {
	const { mdtif } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const viewportRef = React.useRef<HTMLDivElement>(null);
	const { data: preparedDtif, isLoading: isPreparingDtif } = usePreparedDtif(mdtif);

	useSizeCallback(
		viewportRef,
		// Not passing the viewport size as prop to the Canvas or in the DTIF
		// because React is kinda slow updating their states
		(size) => {
			composition?.emitInputEvents('Core', [
				{
					type: 'UpdateCompositionSize',
					size: [size.width, size.height]
				},
				{ type: 'FocusRootNodes' }
			]);
			composition?.update();
		},
		[composition]
	);

	return (
		<div className="grid flex-1 gap-4 overflow-auto p-4 md:grid-cols-2 lg:grid-cols-3">
			{composition != null && mdtif?.scripts != null ? (
				<form className="flex w-full flex-col items-start gap-6">
					{mdtif.scripts.map((script) => (
						<ModificationInput composition={composition} key={script.id} script={script} />
					))}
				</form>
			) : (
				<Skeleton className="h-full w-full rounded-none" />
			)}

			<div className="bg-muted/50 relative flex h-full min-h-[50vh] flex-col overflow-hidden rounded-xl lg:col-span-2">
				{isPreparingDtif || preparedDtif == null ? (
					<Skeleton className="h-full w-full rounded-none" />
				) : (
					<Viewport
						dtif={preparedDtif}
						onLoadedComposition={setComposition}
						viewportRef={viewportRef}
					/>
				)}
				<Badge className="absolute right-3 top-3 bg-white" variant="outline">
					Preview
				</Badge>
			</div>
		</div>
	);
};

export interface TFieldBasedEditorProps {
	mdtif?: TMdtifComposition;
}
