import { describe, it } from 'vitest';

import { createGoogleClient } from './create-google-client';

describe('createGoogleClient function tests', () => {
	it('should have correct types', async () => {
		const client = createGoogleClient({
			apiKey: 'todo'
		});

		const response = await client.getWebFonts();
		const data = response.unwrap();

		// const response = await client.getFontFileURL('Roboto Serif', {
		// 	fontWeight: 100,
		// 	fontStyle: 'italic'
		// });

		// expect(response).not.toBeNull();
	});
});
