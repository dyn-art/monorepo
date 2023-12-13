/**
 * Extracts specified properties from an object.
 *
 * @param source - The source object to extract properties from.
 * @param properties - Array of property keys to extract.
 * @returns An object containing the specified properties of the source.
 */
export function pickProperties<TObject, TPropertyKeys extends keyof TObject>(
	source: TObject,
	properties: TPropertyKeys[]
): Pick<TObject, TPropertyKeys> {
	return properties.reduce<Partial<Pick<TObject, TPropertyKeys>>>((result, property) => {
		const value = source[property];
		result[property] = value;
		return result;
	}, {}) as Pick<TObject, TPropertyKeys>;
}
