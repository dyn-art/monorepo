// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function
export function jsonFunction<GArgs extends string[]>(
	args: GArgs,
	body: string
): TJsonFunction<GArgs> {
	return {
		args,
		body
	};
}

// eslint-disable-next-line @typescript-eslint/no-shadow, @typescript-eslint/ban-types -- .
export function toFunction(jsonFunction: TJsonFunction): Function | null {
	try {
		// eslint-disable-next-line @typescript-eslint/no-implied-eval, no-new-func -- .
		return new Function(...jsonFunction.args, jsonFunction.body);
	} catch (e) {
		// do nothing
	}
	return null;
}

export interface TJsonFunction<GArgs extends string[] = string[]> {
	args: GArgs;
	body: string;
}
