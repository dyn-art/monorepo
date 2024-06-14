'use client';

import React from 'react';
import { isDtif, isMdtif } from '@dyn/arb-dtif';
import { Editor, FieldBasedEditor } from '@dyn/arb-svg-editor';
import { Container, Skeleton } from '@dyn/ui';

import { useDtifFromClipboard } from './_hooks';

const Page: React.FC = () => {
	const { data: dtif } = useDtifFromClipboard();

	if (dtif == null) {
		return <Skeleton className="h-full w-full rounded-none" />;
	}

	if (isMdtif(dtif)) {
		return (
			<Container size="default" tag="main">
				<FieldBasedEditor mdtif={dtif} />
			</Container>
		);
	}

	if (isDtif(dtif)) {
		return (
			<Container className="h-screen" size="full" tag="main">
				<Editor dtif={dtif} />
			</Container>
		);
	}

	return (
		<div>
			<p>Not Found</p>
		</div>
	);
};

export default Page;
