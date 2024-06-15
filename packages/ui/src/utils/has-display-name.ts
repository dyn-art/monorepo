import React from 'react';

export function hasDisplayName(
	element: unknown,
	displayName: string
): element is React.ReactElement<unknown, string | React.JSXElementConstructor<any>> {
	return (
		React.isValidElement(element) &&
		typeof element.type === 'object' &&
		element.type['displayName'] === displayName
	);
}
