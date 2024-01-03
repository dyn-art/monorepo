// Based on:
// https://github.com/trpc/trpc/blob/main/packages/server/src/core/parser.ts

export interface ParserZodEsque<TInput, TParsedInput> {
	_input: TInput;
	_output: TParsedInput;
}

export interface ParserValibotEsque<TInput, TParsedInput> {
	types?: {
		input: TInput;
		output: TParsedInput;
	};
}

export interface ParserMyZodEsque<TInput> {
	parse: (input: any) => TInput;
}

export interface ParserSuperstructEsque<TInput> {
	create: (input: unknown) => TInput;
}

export type ParserCustomValidatorEsque<TInput> = (input: unknown) => Promise<TInput> | TInput;

export interface ParserYupEsque<TInput> {
	validateSync: (input: unknown) => TInput;
}

export interface ParserScaleEsque<TInput> {
	assert: (value: unknown) => asserts value is TInput;
}

export type ParserWithoutInput<TInput> =
	| ParserCustomValidatorEsque<TInput>
	| ParserMyZodEsque<TInput>
	| ParserScaleEsque<TInput>
	| ParserSuperstructEsque<TInput>
	| ParserYupEsque<TInput>;

export type ParserWithInputOutput<TInput, TParsedInput> =
	| ParserZodEsque<TInput, TParsedInput>
	| ParserValibotEsque<TInput, TParsedInput>;

export type Parser = ParserWithInputOutput<any, any> | ParserWithoutInput<any>;

export type inferParser<TParser extends Parser> = TParser extends ParserWithInputOutput<
	infer $TIn,
	infer $TOut
>
	? {
			in: $TIn;
			out: $TOut;
	  }
	: TParser extends ParserWithoutInput<infer $InOut>
	? {
			in: $InOut;
			out: $InOut;
	  }
	: never;
