'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import { isDtif, isMdtif, type CNV, type TMdtifCanvas } from '@dyn/cnv-dtif';
import { Editor, FieldBasedEditor } from '@dyn/cnv-svg-editor';
import { Container } from '@dyn/ui';

const queryClient = new QueryClient();

export const EditorWrapper: React.FC<TProps> = (props) => {
	const { dtif } = props;

	if (isMdtif(dtif)) {
		return (
			<Container size="default" tag="main">
				<QueryClientProvider client={queryClient}>
					<FieldBasedEditor mdtif={dtif} />
				</QueryClientProvider>
			</Container>
		);
	}

	if (isDtif(dtif)) {
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
	dtif: CNV.DtifCanvas | TMdtifCanvas;
}
