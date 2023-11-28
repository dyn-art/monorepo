import { ResizeCursor, RotateCursor } from './Cursor';

export const MIDDLE_HANDLE_WIDTH = 8; // px
export const MIDDLE_HANDLE_HEIGHT = 16; // px
export const CORNER_HANDLE_WIDTH = 8; // px
export const CORNER_HANDLE_HEIGHT = 8; // px

export function getHandleMetaData(width: number, height: number): THandleMetaData[] {
	return [
		{
			corner: EHandleSide.Top + EHandleSide.Left,
			x: -CORNER_HANDLE_WIDTH / 2,
			y: -CORNER_HANDLE_HEIGHT / 2,
			width: CORNER_HANDLE_WIDTH,
			height: CORNER_HANDLE_HEIGHT,
			rotation: -135,
			resizeHandle: {
				cursor: new ResizeCursor(-135)
			},
			rotationHandle: {
				cursor: new RotateCursor(-135)
			}
		},
		{
			corner: EHandleSide.Top,
			x: width / 2 - MIDDLE_HANDLE_HEIGHT / 2,
			y: -MIDDLE_HANDLE_WIDTH / 2,
			width: MIDDLE_HANDLE_HEIGHT,
			height: MIDDLE_HANDLE_WIDTH, // TODO: work with rotation here
			rotation: 90,
			resizeHandle: {
				cursor: new ResizeCursor(90)
			},
			rotationHandle: false
		},
		{
			corner: EHandleSide.Top + EHandleSide.Right,
			x: width - CORNER_HANDLE_WIDTH / 2,
			y: -CORNER_HANDLE_HEIGHT / 2,
			width: CORNER_HANDLE_WIDTH,
			height: CORNER_HANDLE_HEIGHT,
			rotation: 135,
			resizeHandle: {
				cursor: new ResizeCursor(135)
			},
			rotationHandle: {
				cursor: new RotateCursor(135)
			}
		},
		{
			corner: EHandleSide.Right,
			x: width - MIDDLE_HANDLE_WIDTH / 2,
			y: height / 2 - MIDDLE_HANDLE_HEIGHT / 2,
			width: MIDDLE_HANDLE_WIDTH,
			height: MIDDLE_HANDLE_HEIGHT,
			rotation: 0,
			resizeHandle: {
				cursor: new ResizeCursor(0)
			},
			rotationHandle: false
		},
		{
			corner: EHandleSide.Bottom + EHandleSide.Right,
			x: width - CORNER_HANDLE_WIDTH / 2,
			y: height - CORNER_HANDLE_HEIGHT / 2,
			width: CORNER_HANDLE_WIDTH,
			height: CORNER_HANDLE_HEIGHT,
			rotation: 45,
			resizeHandle: {
				cursor: new ResizeCursor(45)
			},
			rotationHandle: {
				cursor: new RotateCursor(45)
			}
		},
		{
			corner: EHandleSide.Bottom,
			x: width / 2 - MIDDLE_HANDLE_HEIGHT / 2,
			y: height - MIDDLE_HANDLE_WIDTH / 2,
			width: MIDDLE_HANDLE_HEIGHT,
			height: MIDDLE_HANDLE_WIDTH,
			rotation: 90,
			resizeHandle: {
				cursor: new ResizeCursor(90)
			},
			rotationHandle: false
		},
		{
			corner: EHandleSide.Bottom + EHandleSide.Left,
			x: -CORNER_HANDLE_WIDTH / 2,
			y: height - CORNER_HANDLE_HEIGHT / 2,
			width: CORNER_HANDLE_WIDTH,
			height: CORNER_HANDLE_HEIGHT,
			rotation: -45,
			resizeHandle: {
				cursor: new ResizeCursor(-45)
			},
			rotationHandle: {
				cursor: new RotateCursor(-45)
			}
		},
		{
			corner: EHandleSide.Left,
			x: -MIDDLE_HANDLE_WIDTH / 2,
			y: height / 2 - MIDDLE_HANDLE_HEIGHT / 2,
			width: MIDDLE_HANDLE_WIDTH,
			height: MIDDLE_HANDLE_HEIGHT,
			rotation: 0,
			resizeHandle: {
				cursor: new ResizeCursor(0)
			},
			rotationHandle: false
		}
	];
}

export type THandleMetaData = {
	corner: number;
	x: number;
	y: number;
	width: number;
	height: number;
	rotation: number;
	resizeHandle: {
		cursor: ResizeCursor;
	};
	rotationHandle:
		| {
				cursor: RotateCursor;
		  }
		| false;
};

export enum EHandleSide {
	Top = 1,
	Bottom = 2,
	Left = 4,
	Right = 8
}
