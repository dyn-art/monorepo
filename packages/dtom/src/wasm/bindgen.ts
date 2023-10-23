import { Canvas } from '../canvas/Canvas';

(globalThis as any).enqueue_rust_events = function onNewWasmEvents(
	worldId: number,
	events: ToJsEvent[]
) {
	console.log('Received new events from Rust', { worldId, events });
	for (const event of events) {
		Canvas.onWasmEvent(worldId, event);
	}
};

export interface WorldIds {
	main_world_id: number;
	render_world_id: number;
}

export interface ToJsEvent {
	RenderUpdate: RenderUpdate;
}

export interface RenderUpdate {
	entity: number;
	changes: RenderChange[];
}

export type RenderChange = Layout | Composition | Blend | Path | RectangleCorner | Children;

export interface Layout {
	Layout: any;
}
export interface Composition {
	Composition: any;
}
export interface Blend {
	Blend: any;
}
export interface Path {
	Path: PathMixin;
}
export interface RectangleCorner {
	RectangleCorner: any;
}
export interface Children {
	Children: any;
}

export type Vec2 = [number, number];

export interface ArcTo {
	radius: Vec2;
	x_axis_rotation: number;
	large_arc_flag: boolean;
	sweep_flag: boolean;
}

export interface CurveTo {
	control_point_1: Vec2;
	control_point_2: Vec2;
}

export type Command = 'MoveTo' | 'LineTo' | { CurveTo: CurveTo } | 'ClosePath' | { ArcTo: ArcTo };

export interface Anchor {
	position: Vec2;
	command: Command;
}

export interface PathMixin {
	vertices: Anchor[];
}
