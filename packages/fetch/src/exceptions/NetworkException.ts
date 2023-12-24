import { ServiceException } from './ServiceException';

export class NetworkException extends ServiceException {
	public readonly throwable?: Error;

	constructor(code: string, options: TNetworkExceptionOptions = {}) {
		const { throwable, description } = options;
		super(code, {
			description,
			message: `Call to endpoint failed with network error ${code}${
				description != null ? `: ${description}` : '!'
			}`
		});
		this.throwable = throwable;
	}
}

interface TNetworkExceptionOptions {
	description?: string;
	throwable?: Error;
}
