'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import { isDtif, isMdtif, type ARB, type TMdtifArtboard } from '@dyn/arb-dtif';
import { Editor, FieldBasedEditor } from '@dyn/arb-svg-editor';
import { LayoutWrapper } from '@dyn/ui';

const queryClient = new QueryClient();

export const EditorWrapper: React.FC<TProps> = (props) => {
	const { dtif } = props;

	if (isMdtif(dtif)) {
		return (
			<LayoutWrapper size="default" asChild>
				<main>
					<QueryClientProvider client={queryClient}>
						<FieldBasedEditor mdtif={dtif} />
					</QueryClientProvider>
				</main>
			</LayoutWrapper>
		);
	}

	if (isDtif(dtif)) {
		return (
			<LayoutWrapper size="full" className="h-screen" asChild>
				<main>
					<QueryClientProvider client={queryClient}>
						<Editor dtif={dtif} />
					</QueryClientProvider>
				</main>
			</LayoutWrapper>
		);
	}

	return null;
};

interface TProps {
	dtif: ARB.DtifArtboard | TMdtifArtboard;
}
