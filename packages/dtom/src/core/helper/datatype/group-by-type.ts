// A generic type representing the structure we expect: a type field and the rest of the object
export type TypeWithDiscriminator<T extends string> = { type: T } & Record<string, unknown>;

// This type will map each unique 'type' value to an array of objects that have that type
export type GroupedByType<T extends TypeWithDiscriminator<string>> = {
	[K in T['type']]: Extract<T, { type: K }>[];
};

// groupByType function
export function groupByType<T extends TypeWithDiscriminator<string>[]>(
	elements: T
): GroupedByType<T[number]> {
	const grouped = elements.reduce<Partial<GroupedByType<T[number]>>>(
		(acc: Partial<GroupedByType<T[number]>>, element) => {
			const typeKey = element.type as T[number]['type'];
			const currentElements = acc[typeKey] || [];
			return { ...acc, [typeKey]: [...currentElements, element] };
		},
		{}
	);

	return grouped as GroupedByType<T[number]>;
}
