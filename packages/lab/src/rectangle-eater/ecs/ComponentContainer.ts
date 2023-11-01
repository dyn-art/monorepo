import type { Component, ComponentClass } from './Component';

/**
 * This custom container is so that calling code can provide the
 * component *instance* when adding (e.g., add(new Position(...))), and
 * provide the Component *class* otherwise (e.g., get(Position),
 * has(Position), delete(Position)).
 *
 * We also use two different types to refer to the component's class:
 * `Function` and `ComponentClass<T>`. We use `Function` in most cases
 * because it is simpler to write. We use `ComponentClass<T>` in the
 * `get()` method, when we want TypeScript to know the type of the
 * instance that is returned. Just think of these both as referring to
 * the same thing: the underlying class of the component.
 *
 * You might notice a footgun here: code that gets this object can
 * directly modify the components inside (with add(...) and delete(...)).
 * This would screw up our ECS bookkeeping of mapping systems to
 * entities! We'll fix this later by only returning callers a view onto
 * the components that can't change them.
 */
export class ComponentContainer {
	// Map of Component class to Component instance
	// e.g. Position -> Position {x: 0, y: 0}
	private _components = new Map<Function, Component>();

	constructor(components: Component[] = []) {
		for (const component of components) {
			this.add(component);
		}
	}

	public add(component: Component): void {
		this._components.set(component.constructor, component);
	}

	public get<T extends Component>(componentClass: ComponentClass<T>): T {
		return this._components.get(componentClass) as T;
	}

	public has(componentClass: Function): boolean {
		return this._components.has(componentClass);
	}

	public hasAll(componentClasses: Iterable<Function>): boolean {
		for (const cls of componentClasses) {
			if (!this._components.has(cls)) {
				return false;
			}
		}
		return true;
	}

	public delete(componentClass: Function): void {
		this._components.delete(componentClass);
	}
}
