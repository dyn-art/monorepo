import type { COMP } from '../comp';

export function isValidDTIF(composition: COMP.DTIFComposition): boolean {
	// Check for required properties
	if (!composition.name || typeof composition.name !== 'string') {
		return false;
	}
	if (!composition.width || typeof composition.width !== 'number' || composition.width <= 0) {
		return false;
	}

	if (!composition.height || typeof composition.height !== 'number' || composition.height <= 0) {
		return false;
	}

	if (!composition.rootNodeId || typeof composition.rootNodeId !== 'string') {
		return false;
	}

	// Check for non-empty nodes and paints
	if (Object.keys(composition.nodes).length === 0) {
		return false;
	}
	if (Object.keys(composition.paints).length === 0) {
		return false;
	}

	return true;
}
