export function getObjectPropertyKeys(obj: Record<string, any>): string[] {
	const properties: string[] = [];
	for (const key in obj) {
		properties.push(key);
	}
	return properties;
}
