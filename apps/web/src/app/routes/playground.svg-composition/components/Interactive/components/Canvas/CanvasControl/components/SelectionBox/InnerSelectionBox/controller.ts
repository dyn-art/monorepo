import { Vec2 } from '@dyn/svg-composition';

import { ResizeCursor, RotateCursor } from './Cursor';

export const RESIZE_MIDDLE_HANDLE_WIDTH = 16; // px
export const RESIZE_MIDDLE_HANDLE_HEIGHT = 8; // px
export const RESIZE_CORNER_HANDLE_WIDTH = 8; // px
export const RESIZE_CORNER_HANDLE_HEIGHT = 8; // px
export const RESIZE_POINTER_AREA_OFFSET = 5; // px
export const ROTATE_HANDLE_SIZE = 20; // px

export function getHandleMetaData(width: number, height: number): THandleMetaData[] {
	return [
		{
			corner: EHandleSide.Top + EHandleSide.Left,
			position: [-RESIZE_CORNER_HANDLE_WIDTH / 2, -RESIZE_CORNER_HANDLE_HEIGHT / 2],
			resizeHandle: {
				width: RESIZE_CORNER_HANDLE_WIDTH,
				height: RESIZE_CORNER_HANDLE_HEIGHT,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(-135)
			},
			rotateHandle: {
				size: ROTATE_HANDLE_SIZE,
				offset: [
					-ROTATE_HANDLE_SIZE - RESIZE_POINTER_AREA_OFFSET,
					-ROTATE_HANDLE_SIZE - RESIZE_POINTER_AREA_OFFSET
				],
				cursor: new RotateCursor(90)
			}
		},
		{
			corner: EHandleSide.Top,
			position: [width / 2 - RESIZE_MIDDLE_HANDLE_WIDTH / 2, -RESIZE_MIDDLE_HANDLE_HEIGHT / 2],
			resizeHandle: {
				width: RESIZE_MIDDLE_HANDLE_WIDTH,
				height: RESIZE_MIDDLE_HANDLE_HEIGHT,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(90)
			},
			rotateHandle: false
		},
		{
			corner: EHandleSide.Top + EHandleSide.Right,
			position: [width - RESIZE_CORNER_HANDLE_WIDTH / 2, -RESIZE_CORNER_HANDLE_HEIGHT / 2],
			resizeHandle: {
				width: RESIZE_CORNER_HANDLE_WIDTH,
				height: RESIZE_CORNER_HANDLE_HEIGHT,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(135)
			},
			rotateHandle: {
				size: ROTATE_HANDLE_SIZE,
				offset: [
					RESIZE_CORNER_HANDLE_WIDTH + RESIZE_POINTER_AREA_OFFSET,
					-ROTATE_HANDLE_SIZE - RESIZE_POINTER_AREA_OFFSET
				],
				cursor: new RotateCursor(180)
			}
		},
		{
			corner: EHandleSide.Right,
			position: [
				width - RESIZE_MIDDLE_HANDLE_HEIGHT / 2,
				height / 2 - RESIZE_MIDDLE_HANDLE_WIDTH / 2
			],
			resizeHandle: {
				width: RESIZE_MIDDLE_HANDLE_HEIGHT,
				height: RESIZE_MIDDLE_HANDLE_WIDTH,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(0)
			},
			rotateHandle: false
		},
		{
			corner: EHandleSide.Bottom + EHandleSide.Right,
			position: [width - RESIZE_CORNER_HANDLE_WIDTH / 2, height - RESIZE_CORNER_HANDLE_HEIGHT / 2],
			resizeHandle: {
				width: RESIZE_CORNER_HANDLE_WIDTH,
				height: RESIZE_CORNER_HANDLE_HEIGHT,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(45)
			},
			rotateHandle: {
				size: ROTATE_HANDLE_SIZE,
				offset: [
					RESIZE_CORNER_HANDLE_WIDTH + RESIZE_POINTER_AREA_OFFSET,
					RESIZE_MIDDLE_HANDLE_HEIGHT + RESIZE_POINTER_AREA_OFFSET
				],
				cursor: new RotateCursor(270)
			}
		},
		{
			corner: EHandleSide.Bottom,
			position: [
				width / 2 - RESIZE_MIDDLE_HANDLE_WIDTH / 2,
				height - RESIZE_MIDDLE_HANDLE_HEIGHT / 2
			],
			resizeHandle: {
				width: RESIZE_MIDDLE_HANDLE_WIDTH,
				height: RESIZE_MIDDLE_HANDLE_HEIGHT,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(90)
			},
			rotateHandle: false
		},
		{
			corner: EHandleSide.Bottom + EHandleSide.Left,
			position: [-RESIZE_CORNER_HANDLE_WIDTH / 2, height - RESIZE_CORNER_HANDLE_HEIGHT / 2],
			resizeHandle: {
				width: RESIZE_CORNER_HANDLE_WIDTH,
				height: RESIZE_CORNER_HANDLE_HEIGHT,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(135)
			},
			rotateHandle: {
				size: ROTATE_HANDLE_SIZE,
				offset: [
					-ROTATE_HANDLE_SIZE - RESIZE_POINTER_AREA_OFFSET,
					RESIZE_CORNER_HANDLE_HEIGHT + RESIZE_POINTER_AREA_OFFSET
				],
				cursor: new RotateCursor(360)
			}
		},
		{
			corner: EHandleSide.Left,
			position: [-RESIZE_MIDDLE_HANDLE_HEIGHT / 2, height / 2 - RESIZE_MIDDLE_HANDLE_WIDTH / 2],
			resizeHandle: {
				width: RESIZE_MIDDLE_HANDLE_HEIGHT,
				height: RESIZE_MIDDLE_HANDLE_WIDTH,
				pointerAreaOffset: RESIZE_POINTER_AREA_OFFSET,
				cursor: new ResizeCursor(0)
			},
			rotateHandle: false
		}
	];
}

export type THandleMetaData = {
	corner: number;
	position: Vec2;
	resizeHandle: {
		width: number;
		height: number;
		pointerAreaOffset: number;
		cursor: ResizeCursor;
	};
	rotateHandle:
		| {
				offset: Vec2;
				size: number;
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
