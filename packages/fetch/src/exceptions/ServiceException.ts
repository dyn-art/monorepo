export class ServiceException extends Error {
	public readonly code: string;
	public readonly description?: string;

	constructor(code: string, options: TServiceExceptionOptions = {}) {
		const { description, message } = options;
		super(message ?? `${code}${description != null ? `: ${description}` : '!'}`);
		this.code = code;
		this.description = description;
	}
}

interface TServiceExceptionOptions {
	description?: string;
	message?: string;
}
