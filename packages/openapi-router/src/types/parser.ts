export interface TParserZodEsque<GResult> {
	parse: (input: unknown) => GResult;
}

export type TParserCustomValidatorEsque<GResult> = (input: unknown) => Promise<GResult> | GResult;

export interface TParserYupEsque<GResult> {
	validateSync: (input: unknown) => GResult;
}

export type TParserEsque<GResult> =
	| TParserCustomValidatorEsque<GResult>
	| TParserZodEsque<GResult>
	| TParserYupEsque<GResult>;

export type TParserSchema<T> = {
	[P in keyof Required<T>]: TParserEsque<T[P]>;
};

export function isTParserZodEsque<GResult>(
	validator: unknown
): validator is TParserZodEsque<GResult> {
	return (
		typeof validator === 'object' &&
		validator != null &&
		'parse' in validator &&
		typeof validator.parse === 'function'
	);
}

export function isTParserCustomValidatorEsque<GResult>(
	validator: unknown
): validator is TParserCustomValidatorEsque<GResult> {
	return validator != null && typeof validator === 'function';
}

export function isTParserYupEsque<GResult>(
	validator: unknown
): validator is TParserYupEsque<GResult> {
	return (
		typeof validator === 'object' &&
		validator != null &&
		'validateSync' in validator &&
		typeof validator.validateSync === 'function'
	);
}

// Inspiration:
// https://github.com/trpc/trpc/blob/main/packages/server/src/core/parser.ts

// export interface ParserZodEsque<TInput, TParsedInput> {
// 	_input: TInput;
// 	_output: TParsedInput;
// }

// export interface ParserValibotEsque<TInput, TParsedInput> {
// 	types?: {
// 		input: TInput;
// 		output: TParsedInput;
// 	};
// }

// export interface ParserMyZodEsque<TInput> {
// 	parse: (input: any) => TInput;
// }

// export interface ParserSuperstructEsque<TInput> {
// 	create: (input: unknown) => TInput;
// }

// export type ParserCustomValidatorEsque<TInput> = (input: unknown) => Promise<TInput> | TInput;

// export interface ParserYupEsque<TInput> {
// 	validateSync: (input: unknown) => TInput;
// }

// export interface ParserScaleEsque<TInput> {
// 	assert: (value: unknown) => asserts value is TInput;
// }

// export type ParserWithoutInput<TInput> =
// 	| ParserCustomValidatorEsque<TInput>
// 	| ParserMyZodEsque<TInput>
// 	| ParserScaleEsque<TInput>
// 	| ParserSuperstructEsque<TInput>
// 	| ParserYupEsque<TInput>;

// export type ParserWithInputOutput<TInput, TParsedInput> =
// 	| ParserZodEsque<TInput, TParsedInput>
// 	| ParserValibotEsque<TInput, TParsedInput>;

// export type Parser = ParserWithInputOutput<any, any> | ParserWithoutInput<any>;

// export type inferParser<TParser extends Parser> = TParser extends ParserWithInputOutput<
// 	infer $TIn,
// 	infer $TOut
// >
// 	? {
// 			in: $TIn;
// 			out: $TOut;
// 	  }
// 	: TParser extends ParserWithoutInput<infer $InOut>
// 	? {
// 			in: $InOut;
// 			out: $InOut;
// 	  }
// 	: never;
