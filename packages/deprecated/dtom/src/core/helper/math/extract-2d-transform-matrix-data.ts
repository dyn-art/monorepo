export function extract2DTransformMatrixData(matrix: T3x3Matrix): T2DMatrixData {
	// | a d tx |
	// | b e ty |
	// | c f j |
	// [ a, b, c, d, e, f, tx, ty, i ]
	const [a, b, , d, e, , tx, ty] = matrix;

	// Extract rotation
	const rotation = Math.atan2(d, a);

	// Extract scale values (scaleX and scaleY)
	// Use the Euclidean norm (length) of each basis vector
	const scaleX = Math.sqrt(a ** 2 + b ** 2);
	const scaleY = Math.sqrt(d ** 2 + e ** 2);

	return {
		tx,
		ty,
		scaleX,
		scaleY,
		rotation: rotation * (180 / Math.PI) // Convert rotation from radians to degrees
	};
}

export interface T2DMatrixData {
	tx: number;
	ty: number;
	scaleX: number;
	scaleY: number;
	rotation: number;
}

export type T3x3Matrix = [number, number, number, number, number, number, number, number, number];
