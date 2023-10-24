import { Composition } from '../composition';

(globalThis as any).enqueue_rust_events = function onNewWasmEvents(
	worldId: number,
	events: ToJsEvent[]
) {
	console.log('Received new events from Rust', { worldId, events });
	for (const event of events) {
		Composition.onWasmEvent(worldId, event);
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
	node_type: 'Rectangle' | 'Group' | 'Frame';
}

export type RenderChange =
	| LayoutChange
	| CompositionChange
	| BlendChange
	| Path
	| RectangleCornerChange
	| ChildrenChange;

export interface LayoutChange {
	Layout: any;
}
export interface CompositionChange {
	Composition: any;
}
export interface BlendChange {
	Blend: any;
}
export interface Path {
	Path: PathMixin;
}
export interface RectangleCornerChange {
	RectangleCorner: any;
}
export interface ChildrenChange {
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
