export class ContinuousId {
	private static nextIdValue = 0;
	private id: number;

	constructor(id: number) {
		this.id = id;
	}

	public static get ZERO(): ContinuousId {
		return new ContinuousId(0);
	}

	public static nextId(): ContinuousId {
		return new ContinuousId(this.nextIdValue++);
	}

	public toNumber(): number {
		return this.id;
	}
}
