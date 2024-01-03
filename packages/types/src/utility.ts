export type Unarray<T> = T extends (infer U)[] ? U : T;

export type Unwrap<T> = T extends Promise<infer U> ? U : T;

// https://fettblog.eu/typescript-union-to-intersection/
export type TUnionToIntersection<T> = (T extends any ? (x: T) => any : never) extends (
	x: infer R
) => any
	? R
	: never;

export type TPrimitive = boolean | number | string;

export type TErrorMessage<GMessage extends string> = GMessage;
