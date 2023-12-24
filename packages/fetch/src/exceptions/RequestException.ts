import { ServiceException } from './ServiceException';

export class RequestException<GData = any> extends ServiceException {
	public readonly status: number;
	public readonly response?: Response;
	public readonly data?: GData;

	constructor(code: string, status: number, options: TRequestExceptionOptions<GData> = {}) {
		const { description, response, data } = options;
		super(code, {
			description,
			message: `Call to endpoint failed with status ${status} and error code ${code}${
				description != null ? `: ${description}` : '!'
			}`
		});
		this.status = status;
		this.response = response;
		this.data = data;
	}
}

interface TRequestExceptionOptions<GData> {
	description?: string;
	data?: GData;
	response?: Response;
}
