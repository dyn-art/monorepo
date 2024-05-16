export type THexColor = `#${string}`;
export type TRgbColor = [number, number, number];
export type TColor = THexColor | TRgbColor;

/**
 * A 3x3 column majfor matrix.
 */
export type TMat3 = [[number, number, number], [number, number, number], [number, number, number]];

/**
 * A 2-dimensional vector.
 */
export type TVec2 = [number, number];
