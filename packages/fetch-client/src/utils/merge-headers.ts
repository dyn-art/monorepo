/**
 * Merges two sets of headers into a single record.
 *
 * @param headers1 - The first set of headers.
 * @param headers2 - The second set of headers to merge with the first.
 * @returns A record containing the merged headers.
 */
export function mergeHeaders(
	headers1?: RequestInit['headers'],
	headers2?: RequestInit['headers']
): Headers {
	const merged = new Headers(headers1);

	if (headers2) {
		// Add or overwrite the second set of headers to the merged headers
		if (headers2 instanceof Headers) {
			headers2.forEach((value, key) => {
				merged.set(key, value);
			});
		} else {
			// Handles both array and object formats
			Object.entries(headers2 as Record<string, string | readonly string[]>).forEach(
				([key, value]) => {
					if (Array.isArray(value)) {
						// Append each value if it's an array,
						// instead of replacing the existing value
						value.forEach((val) => {
							merged.append(key, val);
						});
					} else {
						merged.set(key, value as string);
					}
				}
			);
		}
	}

	return merged;
}
