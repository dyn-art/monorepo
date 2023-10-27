export const VERSION = '1.1';
export const NS = 'http://www.w3.org/2000/svg';
export const XLINK = 'http://www.w3.org/1999/xlink';

// Create a namespaced SVG element
export function createSVGElement(tag: TSVGTagNames, attributes: TSVGAttributes = {}): SVGElement {
	const element = document.createElementNS(NS, tag);
	console.log('createSVGElement', { element });
	if (tag === 'svg' && attributes.version == null) {
		attributes.version = VERSION;
	}
	if (Object.keys(attributes).length > 0) {
		setAttributes(element, attributes);
	}
	return element;
}

// Add attributes to SVG element
export function setAttributes(element: SVGElement, attributes: TSVGAttributes) {
	for (const [key, value] of Object.entries(attributes)) {
		if (key.includes('href')) {
			element.setAttributeNS(XLINK, key, value);
		} else {
			element.setAttribute(key, value);
		}
	}
}

// Remove attributes from SVG element
export function removeAttributes(element: Element, attributeKeys: string[]) {
	for (const attr of attributeKeys) {
		element.removeAttribute(attr);
	}
}

// Add styles to SVG element
export function setStyles(element: SVGElement, styles: TSVGStyles) {
	for (const [key, value] of Object.entries(styles)) {
		element.style.setProperty(key, value);
	}
}

export type TSVGAttributes = Record<string, string>;
export type TSVGStyles = Record<string, string>;

export type TSVGTagNames =
	| 'svg'
	| 'circle'
	| 'ellipse'
	| 'line'
	| 'path'
	| 'polygon'
	| 'polyline'
	| 'rect'
	| 'text'
	| 'use'
	| 'defs'
	| 'g'
	| 'symbol';
