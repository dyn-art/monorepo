'use client';

import React from 'react';
import { Skeleton } from '@dyn/ui';

import { UniversalEditor } from '../_components/UniversalEditor';
import { useDtifFromClipboard } from './_hooks';

const Page: React.FC = () => {
	const { data: dtif } = useDtifFromClipboard(); // TODO: No query client set yet

	if (dtif == null) {
		return <Skeleton className="h-full w-full rounded-none" />;
	}

	return <UniversalEditor dtif={dtif} />;
};

export default Page;
