// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function
export function createJsonFunction<GArgs extends string[]>(
	args: GArgs,
	body: string
): TJsonFunction<GArgs> {
	return {
		args,
		body
	};
}

// Only run in restricted scope (e.g. iframe)!
export function toFunction(jsonFunction: TJsonFunction): Function {
	// eslint-disable-next-line @typescript-eslint/no-implied-eval, no-new-func -- .
	return new Function(...jsonFunction.args, jsonFunction.body);
}

export interface TJsonFunction<GArgs extends string[] = string[]> {
	args: GArgs;
	body: string;
}
