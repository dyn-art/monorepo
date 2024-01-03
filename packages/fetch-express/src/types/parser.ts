import type {
	ZodArray,
	ZodBoolean,
	ZodDate,
	ZodLiteral,
	ZodNumber,
	ZodObject,
	ZodOptional,
	ZodString,
	ZodUnion
} from 'zod';

// Idea 1

type TBaseZodType<T> = T extends string
	? ZodString
	: T extends number
	? ZodNumber
	: T extends boolean
	? ZodBoolean
	: T extends Date
	? ZodDate
	: T extends (infer U)[]
	? ZodArray<TBaseZodType<U>>
	: T extends readonly (infer U)[]
	? ZodArray<TBaseZodType<U>>
	: T extends object
	? ZodObject<{ [K in keyof T]: TBaseZodType<T[K]> }>
	: never;

type TZodType<T> = T extends any[] | readonly any[]
	? ZodArray<TBaseZodType<T[number]>>
	: T extends (...args: any) => any
	? TBaseZodType<ReturnType<T>>
	: T extends boolean | string | number | Date | object
	? TBaseZodType<T>
	: T extends infer U | infer V
	? ZodUnion<[TUnionToZod<U>, TUnionToZod<V>]>
	: never;

type TLiteralToZod<T> = T extends infer U ? ZodLiteral<U> : never;

type TUnionToZod<T> = T extends infer U ? ZodUnion<[TZodType<U>]> : never;

type TOptionalToZod<T> = T extends infer U ? ZodOptional<TZodType<U>> : never;

type ToZodSchema<T> = {
	// Mapping non-optional properties
	[P in keyof T as undefined extends T[P] ? never : P]: TBaseZodType<T[P]>;
} & {
	// Mapping optional properties
	[P in keyof T as undefined extends T[P] ? P : never]: TOptionalToZod<Exclude<T[P], undefined>>;
};

interface ExampleType {
	name?: string;
	age: number;
	isStudent: boolean;
	birthDate: Date;
	hobbies?: string[];
	// getGreeting: () => string;
	status?: 'active' | 'inactive'; // Now correctly handled as optional
}

type ExampleZodType = ToZodSchema<ExampleType>;

// TODO: Idea 2 - Just enforce this type lol, no complex Zod type mapping and inferance
// as the parse function in the Zod type should look like the below parse type
// and it only does so if the correct Zod schema is defined

export interface ParserZodEsque<TInput> {
	parse: (input: any) => TInput;
	_type: TInput;
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
