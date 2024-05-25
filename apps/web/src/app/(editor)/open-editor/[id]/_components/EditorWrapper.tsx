'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import {
	isDtifComposition,
	isMdtifComposition,
	type COMP,
	type TMdtifComposition
} from '@dyn/comp-dtif';
import { Editor, FieldBasedEditor } from '@dyn/comp-svg-editor';
import { Container } from '@dyn/ui';

const queryClient = new QueryClient();

export const EditorWrapper: React.FC<TProps> = (props) => {
	const { dtif } = props;

	if (isMdtifComposition(dtif)) {
		return (
			<Container size="default" tag="main">
				<QueryClientProvider client={queryClient}>
					<FieldBasedEditor mdtif={dtif} />
				</QueryClientProvider>
			</Container>
		);
	}

	if (isDtifComposition(dtif)) {
		return (
			<Container className="h-screen" size="full" tag="main">
				<QueryClientProvider client={queryClient}>
					<Editor dtif={dtif} />
				</QueryClientProvider>
			</Container>
		);
	}

	return null;
};

interface TProps {
	dtif: COMP.DtifComposition | TMdtifComposition;
}
