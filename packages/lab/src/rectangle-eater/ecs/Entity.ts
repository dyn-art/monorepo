/**
 * An entity is basically just an ID. This is used to look up its associated
 * Components.
 */
export class Entity {
	private _index: number;
	private _generation: number;

	constructor(index: number, generation: number) {
		this._index = index;
		this._generation = generation;
	}

	public get index(): number {
		return this._index;
	}

	public get generation(): number {
		return this._generation;
	}
}
