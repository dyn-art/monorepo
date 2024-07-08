import { ServiceError } from '@ibg/openapi-router';
import type express from 'express';
import { type components } from '@dyn/types/core';

export function errorMiddleware(
	err: unknown,
	_req: express.Request,
	res: express.Response,
	_next: express.NextFunction
): void {
	let statusCode = 500;
	const jsonResponse: components['schemas']['ServiceError'] = {
		error_code: '#ERR_UNKNOWN',
		error_description: null,
		error_uri: null,
		additional_errors: []
	};

	// Handle application-specific errors (instances of AppError)
	if (err instanceof ServiceError) {
		statusCode = err.status;
		jsonResponse.error_code = err.code;
		jsonResponse.error_description = err.description;
		jsonResponse.error_uri = err.uri ?? null;
		jsonResponse.additional_errors = err.additionalErrors as any;
	}

	// Handle unknown errors
	else if (typeof err === 'object' && err != null) {
		if ('message' in err && typeof err.message === 'string') {
			jsonResponse.error_description = err.message;
		}
		if ('code' in err && typeof err.code === 'string') {
			jsonResponse.error_code = err.code;
		}
	} else {
		jsonResponse.error_description = 'An unknown error occurred!';
	}

	res.status(statusCode).json(jsonResponse);
}
