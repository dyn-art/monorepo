import { shortId } from '@ibg/utils';
import { SvgCnvHandle } from '@/rust/dyn-cnv-svg-builder-api';
import type {
	CanvasChangeOutputEvent,
	ComponentChange,
	CoreInputEvent,
	CursorChangeOutputEvent,
	DtifCanvas,
	Entity,
	ExecuteLuaScriptInputEvent,
	InteractionInputEvent,
	InteractionModeChangeOutputEvent,
	InteractionToolChangeOutputEvent,
	LuaScriptError,
	SelectionChangeOutputEvent,
	Size,
	SvgCnvInputEvent,
	SvgCnvOutputEvent,
	SvgElementChangesOutputEvent,
	Viewport,
	WatchableComponentVariant,
	WatchedEntityChangesOutputEvent
} from '@/rust/dyn-cnv-svg-builder-api/bindings';

import { SvgBuilder, type TSvgBuilderOptions } from './SvgBuilder';

export class Canvas {
	private readonly _svgCnvHandle: SvgCnvHandle;
	private _builder: SvgBuilder;

	private _size: Size;
	private _viewport: Viewport;
	private _selectedEntities: Entity[];

	private _inputEventQueue: SvgCnvInputEvent[] = [];
	private _watchedOutputEventCallbackMap: TWatchedOutputEventsCallbackMap = {};

	// Interaction events debounce
	private debounceTimeout: number | null = null;
	private readonly debounceDelay: number = 100; // ms

	constructor(config: TCanvasConfig) {
		const { dtif, interactive = false } = config;
		this._svgCnvHandle = SvgCnvHandle.create(dtif, interactive);
		this._builder = new SvgBuilder(this, config);
		this.watchOutputEvent('CanvasChange', (event) => {
			this._size = event.size;
			this._viewport = event.viewport;
		});
		this.watchOutputEvent('SelectionChange', (event) => {
			this._selectedEntities = event.selectedEntities;
		});
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	public get size(): Readonly<Size> {
		return this._size;
	}

	public get viewport(): Readonly<Viewport> {
		return this._viewport;
	}

	public get selectedEntities(): Readonly<Entity[]> {
		return this._selectedEntities;
	}

	public get builder(): SvgBuilder {
		return this._builder;
	}

	// =========================================================================
	// Lifecycle
	// =========================================================================

	public update(): void {
		const outputEvents: SvgCnvOutputEvent[] = this._svgCnvHandle.update(
			this._inputEventQueue.splice(0, this._inputEventQueue.length)
		);

		// Handle output events
		for (const event of outputEvents) {
			this.handleOutputEvent(event.type, event);
		}
	}

	public unmount(): void {
		this._svgCnvHandle.free();
		this._builder.clear();
		this._watchedOutputEventCallbackMap = {};
		this._inputEventQueue = [];
	}

	// =========================================================================
	// Events
	// =========================================================================

	private handleOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		event: TOutputEventTypeMap[GEventType]
	): void {
		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (!Array.isArray(callbacks)) {
			return;
		}

		callbacks.forEach((entry) => {
			entry.callback(event);
		});
	}

	public watchOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		callback: TWatchedOutputEventCallback<GEventType>
	): () => void {
		const id = shortId();
		const entry: TWatchedOutputEventCallbackEntry<GEventType> = { id, callback };

		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (Array.isArray(callbacks)) {
			callbacks.push(entry);
		} else {
			// @ts-expect-error -- Entry is of type GEventType
			this._watchedOutputEventCallbackMap[eventType] = [entry];
		}

		// Return an unregister function
		return () => {
			this.unregisterOutputEventCallback(eventType, id);
		};
	}

	private unregisterOutputEventCallback<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		id: string
	): void {
		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (!Array.isArray(callbacks)) {
			return;
		}

		// @ts-expect-error -- Array has only elements of type GEventType
		this._watchedOutputEventCallbackMap[eventType] = callbacks.filter((entry) => entry.id !== id);
	}

	public watchEntity(
		entity: Entity,
		toWatchComponents: WatchableComponentVariant[],
		callback: TWatchEntityCallback,
		initialValue = true
	): () => void {
		const intialChanges = this._svgCnvHandle.watchEntity(entity, toWatchComponents, initialValue);

		// Apply intal changes if found
		if (initialValue && Array.isArray(intialChanges)) {
			callback(entity, intialChanges);
		}

		// Register callback
		const unregister = this.watchOutputEvent('WatchedEntityChange', (event) => {
			if (event.entity === entity) {
				callback(entity, event.changes);
			}
		});

		return () => {
			unregister();
			this._svgCnvHandle.unregisterEntityCallback(entity);
		};
	}

	public emitInputEvent<GEventType extends keyof TInputEventTypeMap>(
		eventType: GEventType,
		event: TInputEventTypeMap[GEventType],
		debounce = true
	): void {
		this._inputEventQueue.push({ type: eventType, event } as SvgCnvInputEvent);

		// Delay update call, resetting timer on new events within debounceDelay
		if (eventType === 'Interaction' && this.builder.isCallbackBased) {
			if (this.debounceTimeout != null) {
				clearTimeout(this.debounceTimeout);
			}
			if (debounce) {
				this.debounceTimeout = setTimeout(() => {
					this.update();
				}, this.debounceDelay) as unknown as number;
			} else {
				this.update();
			}
		}
	}

	public emitInputEvents<GEventType extends keyof TInputEventTypeMap>(
		eventType: GEventType,
		events: TInputEventTypeMap[GEventType][],
		debounce = true
	): void {
		for (const event of events) {
			this.emitInputEvent(eventType, event, debounce);
		}
	}

	// =========================================================================
	// Other
	// =========================================================================

	public executeScript(script: ExecuteLuaScriptInputEvent): LuaScriptError | null {
		return this._svgCnvHandle.executeScript(script);
	}

	public logEntityComponentsRaw(rawEntity: number): void {
		this._svgCnvHandle.logEntityComponentsRaw(rawEntity);
	}

	public logEntityComponents(entity: Entity): void {
		this._svgCnvHandle.logEntityComponents(entity);
	}

	public tempDevLog(): void {
		this._svgCnvHandle.tempDevLog();
	}

	public toString(): string | null {
		return this._svgCnvHandle.toString() ?? null;
	}
}

export interface TCanvasConfig extends TSvgBuilderOptions {
	dtif: DtifCanvas;
	interactive?: boolean;
}

export interface TInputEventTypeMap {
	Core: CoreInputEvent;
	Interaction: InteractionInputEvent;
}

export interface TOutputEventTypeMap {
	SvgElementChange: SvgElementChangesOutputEvent;
	WatchedEntityChange: WatchedEntityChangesOutputEvent;
	SelectionChange: SelectionChangeOutputEvent;
	CanvasChange: CanvasChangeOutputEvent;
	InteractionModeChange: InteractionModeChangeOutputEvent;
	InteractionToolChange: InteractionToolChangeOutputEvent;
	CursorChange: CursorChangeOutputEvent;
}

export type TWatchedOutputEventCallback<GEventType extends keyof TOutputEventTypeMap> = (
	event: TOutputEventTypeMap[GEventType]
) => void;

interface TWatchedOutputEventCallbackEntry<GEventType extends keyof TOutputEventTypeMap> {
	id: string;
	callback: TWatchedOutputEventCallback<GEventType>;
}

type TWatchedOutputEventsCallbackMap = {
	[K in SvgCnvOutputEvent['type']]?: TWatchedOutputEventCallbackEntry<K>[];
};

export type TWatchEntityCallback = (entity: Entity, change: ComponentChange[]) => void;
