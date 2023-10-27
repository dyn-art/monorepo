/**
 * Helper function to convert a received Rust enum array into a TypeScript object.
 *
 * @param enumArray - Array of events received from Rust.
 * Each element is an object that represents a Rust enum variant.
 *
 * @returns A grouped object where each key is a Rust enum variant name,
 * and its value is an array of occurrences.
 */

export function transformRustEnumArrayToObject<T extends Record<string, any>>(
	enumArray: T[]
): GroupedRustEnums<T> {
	const groupedEnum: Partial<GroupedRustEnums<T>> = {};
	for (const event of enumArray) {
		const eventType = Object.keys(event)[0] as keyof T;
		const eventData = event[eventType];
		if (!(eventType in groupedEnum)) {
			groupedEnum[eventType as RustEnumKeys<T>] = [];
		}
		groupedEnum[eventType]?.push(eventData);
	}
	return groupedEnum;
}

// Converts a union type to an intersection type.
// E.g., A | B | C becomes A & B & C
type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (k: infer I) => void
	? I
	: never;

// Retrieves the keys that are common across all types in a union.
// Utilizes UnionToIntersection to flatten the union to an intersection of all types, and then gets the keys.
type KeysOfUnion<T> = keyof UnionToIntersection<T>;

// For a given union type T and a key K, maps to the corresponding value type for that key in each type of T.
type DiscriminateUnion<T, K extends KeysOfUnion<T>> = T extends Record<K, infer V> ? V : never;

// Groups each discriminant of a Rust-style tagged union into separate arrays.
// Uses KeysOfUnion to dynamically get union keys and DiscriminateUnion to get the value type for each key.
export type GroupedRustEnums<T> = {
	[K in KeysOfUnion<T>]?: DiscriminateUnion<T, K>[];
};

// Alias for KeysOfUnion, intended to provide a more context-specific name.
export type RustEnumKeys<T> = KeysOfUnion<T>;
