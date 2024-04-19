// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function
export function jsonFunction(args: string[], body: string): TJsonFunction {
	return {
		args,
		body
	};
}

export interface TJsonFunction {
	args: string[];
	body: string;
}
