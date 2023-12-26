import { isObject } from '@dyn/utils';

import { RequestException } from '../exceptions';

export async function mapResponseToRequestException(
	response: Response,
	defaultErrorCode = '#ERR_UNKNOWN'
): Promise<RequestException> {
	try {
		const contentType = response.headers.get('Content-Type');

		let error;
		let errorCode;
		let errorDescription;
		if (contentType && contentType.includes('application/json')) {
			error = await response.json();
			errorCode = getErrorCode(error) ?? defaultErrorCode;
			errorDescription = getErrorDescription(error) ?? undefined;
		} else {
			error = await response.text();
			errorCode = defaultErrorCode;
			errorDescription = error;
		}

		return new RequestException(errorCode, response.status, {
			description: errorDescription,
			data: error as any,
			response
		});
	} catch (error) {
		return new RequestException(defaultErrorCode, response.status, {
			description: 'Error processing response',
			data: error as any,
			response
		});
	}
}

// Helper function to extract error description from various possible fields
function getErrorDescription(data: unknown): string | null {
	if (isObject(data)) {
		return data.error_description || data.error?.toString() || data.message || null;
	}
	return null;
}

// Helper function to extract error code from various possible fields
function getErrorCode(data: unknown): string | null {
	if (isObject(data)) {
		return data.error_code || data.status || data.code || getErrorCode(data.error) || null;
	}
	return null;
}
