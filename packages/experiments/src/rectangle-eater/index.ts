import { defineComponent, Types } from 'bitecs';

function init() {
	const Vector3 = { x: Types.f32, y: Types.f32, z: Types.f32 };
	const Position = defineComponent(Vector3);
}
