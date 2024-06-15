'use client';

import React from 'react';
import { isDtif, isMdtif } from '@dyn/arb-dtif';
import { Editor, FieldBasedEditor } from '@dyn/arb-svg-editor';
import { LayoutWrapper, Skeleton } from '@dyn/ui';

import { useDtifFromClipboard } from './_hooks';

const Page: React.FC = () => {
	const { data: dtif } = useDtifFromClipboard();

	if (dtif == null) {
		return <Skeleton className="h-full w-full rounded-none" />;
	}

	if (isMdtif(dtif)) {
		return (
			<LayoutWrapper size="default" asChild>
				<main>
					<FieldBasedEditor mdtif={dtif} />
				</main>
			</LayoutWrapper>
		);
	}

	if (isDtif(dtif)) {
		return (
			<LayoutWrapper className="h-screen" size="full" asChild>
				<main>
					<Editor dtif={dtif} />
				</main>
			</LayoutWrapper>
		);
	}

	return (
		<div>
			<p>Not Found</p>
		</div>
	);
};

export default Page;
