import { extract2DTransformMatrixData, type T3x3Matrix } from '../math';

export function transformToCSS(
	transform: T3x3Matrix,
	asMatrix = true
): { transform: string; transformOrigin: string } {
	if (asMatrix) {
		// | a d tx |
		// | b e ty |
		// | c f j |
		// [ a, b, c, d, e, f, tx, ty, i ]
		const [a, d, , b, e, , tx, ty] = transform;
		return {
			transform: `matrix(${a}, ${b}, ${d}, ${e}, ${tx}, ${ty})`,
			transformOrigin: '0 0' // top left
		};
	}

	const { rotation, scaleX, scaleY, tx: x, ty: y } = extract2DTransformMatrixData(transform);
	return {
		transform: `translate(${x}px, ${y}px) rotate(${
			// We negate the rotation to correct for Figma's clockwise rotation
			-rotation
		}deg) scale(${scaleX}, ${scaleY})`,
		transformOrigin: '0 0' // top left
	};
}
