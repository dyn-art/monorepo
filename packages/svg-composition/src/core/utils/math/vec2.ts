import type { Vec2 } from '@/rust/dyn_svg_composition_api/bindings';

export function vec2(x: number, y: number): Vec2 {
	return [x, y];
}
