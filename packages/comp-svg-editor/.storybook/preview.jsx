import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';

import '../src/styles/root.scss';

const queryClient = new QueryClient();

/** @type { import('@storybook/react').Preview } */
const preview = {
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i
			}
		}
	},
	decorators: (story) => <QueryClientProvider client={queryClient}>{story()}</QueryClientProvider>
};

export default preview;
