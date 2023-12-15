const supportedNodeTypes = [
	'FRAME',
	'COMPONENT',
	'INSTANCE',
	'GROUP',
	'TEXT',
	'RECTANGLE',
	'LINE',
	'ELLIPSE',
	'POLYGON',
	'STAR',
	'VECTOR',
	'BOOLEAN_OPERATION'
];

export function canBeTransformedToDTIF(type: string): boolean {
	return supportedNodeTypes.includes(type);
}
