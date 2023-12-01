export type TRustEnumKeyArray<T> = T extends { type: infer U } ? U : never;
