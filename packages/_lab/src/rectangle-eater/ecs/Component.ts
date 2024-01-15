/**
 * A Component is a bundle of state. Each instance of a Component is
 * associated with a single Entity.
 *
 * Components have no API to fulfill.
 */
export class Component<TData extends Record<string, unknown> = Record<string, unknown>> {
	private _data: TData;

	constructor(data: TData) {
		this._data = data;
	}
}

/**
 * This type is so functions like the ComponentContainer's get(...) will
 * automatically tell TypeScript the type of the Component returned. In
 * other words, we can say get(Position) and TypeScript will know that an
 * instance of Position was returned. This is amazingly helpful.
 */
export type ComponentClass<T extends Component> = new (...args: any[]) => T;
