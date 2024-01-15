import type { Component } from './Component';
import { ComponentContainer } from './ComponentContainer';
import { Entity } from './Entity';
import type { System } from './System';

export class World {
	private _entities = new Map<Entity, ComponentContainer>();
	private nextEntityId = 0;

	private readonly _systems = new Map<System, Set<Entity>>();

	public spawn(components: Component[] | Component): Entity {
		const entity = new Entity(this.nextEntityId++, 0);
		this._entities.set(
			entity,
			new ComponentContainer(Array.isArray(components) ? components : [components])
		);
		return entity;
	}

	public despawn(entity: Entity): this {
		this._entities.delete(entity);
		this._systems.forEach((entities) => {
			entities.delete(entity);
		});
		return this;
	}

	public addSystem(system: System): this {
		// Checking invariant: Systems should not have an empty
		// components list, or they'll run on every entity. Simply remove
		// or special case this check if you do want a system that runs
		// on everything.
		if (system.componentsRequired.size === 0) {
			console.warn('System not added: empty Components list.', { system });
			return this;
		}

		// Find all entities that have all the components required by this system
		const entities = new Set<Entity>();
		this._entities.forEach((components, entity) => {
			if (components.hasAll(system.componentsRequired)) {
				entities.add(entity);
			}
		});
		this._systems.set(system, entities);

		return this;
	}

	public removeSystem(system: System): this {
		this._systems.delete(system);
		return this;
	}

	public run(): this {
		this._systems.forEach((entities, system) => {
			system.update(entities, this);
		});
		return this;
	}
}
