import type { RenderUpdateEvent } from '@/rust/dyn_composition_api/bindings';

export abstract class Renderer {
	public abstract setSize(width: number, height: number): this;

	public abstract render(events: RenderUpdateEvent[]): this;

	public abstract clear(): this;
}
