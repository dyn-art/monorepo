export type Unarray<T> = T extends (infer U)[] ? U : T;

export type Unwrap<T> = T extends Promise<infer U> ? U : T;
