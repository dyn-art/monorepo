import { AppError } from '@ibg/openapi-router';
import type * as hono from 'hono/types';
import { type StatusCode } from 'hono/utils/http-status';
import { type components } from '@dyn/types/core';

export const errorHandler: hono.ErrorHandler = (err, c) => {
	let statusCode = 500;
	const jsonResponse: components['schemas']['AppErrorDto'] = {
		error_code: '#ERR_UNKNOWN',
		error_description: null,
		error_uri: null,
		additional_errors: []
	};

	// Handle application-specific errors (instances of AppError)
	if (err instanceof AppError) {
		statusCode = err.status;
		jsonResponse.error_code = err.code;
		jsonResponse.error_description = err.description ?? null;
		jsonResponse.error_uri = err.uri ?? null;
		jsonResponse.additional_errors = err.additionalErrors as any;
	}

	// Handle unknown errors
	else if (typeof err === 'object') {
		if ('message' in err && typeof err.message === 'string') {
			jsonResponse.error_description = err.message;
		}
		if ('code' in err && typeof err.code === 'string') {
			jsonResponse.error_code = err.code;
		}
	} else {
		jsonResponse.error_description = 'An unknown error occurred!';
	}

	return c.json(jsonResponse, statusCode as StatusCode);
};
