'use client';

import React from 'react';
import { isDtifComposition, isMdtifComposition } from '@dyn/dtif-comp';
import { Container, Skeleton } from '@dyn/ui';
import { Editor } from '@/components';

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
				<p>Hello World</p>
			</Container>
		);
	}

	return null;
};

export default Page;
