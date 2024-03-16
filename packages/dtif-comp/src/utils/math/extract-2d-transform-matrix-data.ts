export function extract2DTransformMatrixData(matrix: TFlat3x3Matrix | T3x3Matrix): T2DMatrixData {
	const flatMatrix: TFlat3x3Matrix = is3x3Matrix(matrix)
		? [
				matrix[0][0],
				matrix[1][0],
				matrix[2][0],
				matrix[0][1],
				matrix[1][1],
				matrix[2][1],
				matrix[0][2],
				matrix[1][2],
				matrix[2][2]
			]
		: matrix;

	// | a d tx |
	// | b e ty |
	// | c f j |
	// [ a, b, c, d, e, f, tx, ty, i ]
	const [a, b, , d, e, , tx, ty] = flatMatrix;

	// Extract rotation
	const rotationInRadians = Math.atan2(d, a);

	// Extract scale values (scaleX and scaleY)
	// Use the Euclidean norm (length) of each basis vector
	const scaleX = Math.sqrt(a ** 2 + b ** 2);
	const scaleY = Math.sqrt(d ** 2 + e ** 2);

	return {
		tx,
		ty,
		scaleX,
		scaleY,
		rotationInRadians
	};
}

export interface T2DMatrixData {
	tx: number;
	ty: number;
	scaleX: number;
	scaleY: number;
	rotationInRadians: number;
}

// Based on serialized glam Mat3
// | a d tx |
// | b e ty |
// | c f j |
// [ a, b, c, d, e, f, tx, ty, i ]
export type TFlat3x3Matrix = [
	number,
	number,
	number,
	number,
	number,
	number,
	number,
	number,
	number
];
// Based on Figma Transform matrix (https://www.figma.com/plugin-docs/api/Transform/)
// | a d tx |
// | b e ty |
// | c f j |
// [a, d, tx]
// [b, e, ty]
// [c, f, j]
export type T3x3Matrix = [[number, number, number], [number, number, number], [0, 0, 1]];

export function is3x3Matrix(matrix: unknown): matrix is T3x3Matrix {
	return (
		Array.isArray(matrix) &&
		Array.isArray(matrix[0]) &&
		matrix[0].length === 3 &&
		Array.isArray(matrix[1]) &&
		matrix[1].length === 3 &&
		Array.isArray(matrix[2]) &&
		matrix[2].length === 3
	);
}
