/**
 * Recursively replaces `var` properties in a data structure with values from a `placeholders` map.
 *
 * @example
 * ```
 * const originalData = {
 *     name: 'Alice',
 *     details: { age: { var: 'AGE_PLACEHOLDER' }, city: { var: 'CITY_PLACEHOLDER' } },
 *     tags: [{ var: 'AGE_PLACEHOLDER' }, 'student']
 * };
 * const placeholders = { AGE_PLACEHOLDER: 25, CITY_PLACEHOLDER: 'New York' };
 * const newData = deepReplaceVar(originalData, placeholders); // returns { name: 'Alice', details: { age: 25, city: 'New York' }, tags: [25, 'student'] }
 * ```
 */
export function deepReplaceVar<T>(data: T, placeholders: Record<string, any>): T {
	const cloneAndReplace = (item: any): any => {
		// Replace the placeholder with the new value
		if (
			typeof item === 'object' &&
			item?.var != null &&
			Object.prototype.hasOwnProperty.call(placeholders, item.var)
		) {
			return placeholders[item.var];
		} else if (typeof item === 'object' && item != null) {
			// Clone and process each element of the array
			if (Array.isArray(item)) {
				return item.map(cloneAndReplace);
			}

			// Clone and process each property of the object
			return Object.keys(item).reduce<Record<string, any>>((acc, key) => {
				acc[key] = cloneAndReplace(item[key]);
				return acc;
			}, {});
		}

		// Return the item unchanged if it's not an object or the target string
		return item;
	};

	return cloneAndReplace(data);
}
