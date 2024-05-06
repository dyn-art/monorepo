'use client';

import React from 'react';
import { isDtifComposition, isMdtifComposition } from '@dyn/comp-dtif';
import { Container, Skeleton } from '@dyn/ui';
import { Editor, FieldBasedEditor } from '@/components';

import { useDtifFromClipboard } from './_hooks';

const Page: React.FC = () => {
	const { data: dtif } = useDtifFromClipboard();

	if (dtif == null) {
		return <Skeleton className="h-full w-full rounded-none" />;
	}

	if (isDtifComposition(dtif)) {
		return (
			<Container className="h-screen" size="full" tag="main">
				<Editor dtif={dtif} />
			</Container>
		);
	}

	if (isMdtifComposition(dtif)) {
		return (
			<Container size="default" tag="main">
				<FieldBasedEditor mdtif={dtif} />
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
