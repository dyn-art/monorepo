// =============================================================================
// State
// =============================================================================

export type TState<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]> = {
	_value: GValue;
} & TSelectFeatures<GValue, GSelectedFeatureKeys>;

// =============================================================================
// Features
// =============================================================================

export interface TCoreFeatures<GValue> {
	get: {
		/**
		 * Retrieves the current state value.
		 *
		 * Example usage:
		 * ```js
		 * const currentState = MY_STATE.get();
		 * ```
		 *
		 * @returns The current state value of type `GValue`.
		 */
		get: () => GValue;
	};
	set: {
		/**
		 * Updates the state value.
		 *
		 * Example usage:
		 * ```js
		 * MY_STATE.set("Hello World");
		 * ```
		 *
		 * @param newValue - The new value to set for the state, of type `GValue`.
		 */
		set: (newValue: GValue) => void;
	};
	listen: {
		_listeners: TListener<GValue>[];
		/**
		 * Subscribes to state changes without immediately invoking the callback.
		 * Use this to listen for changes that occur after the subscription.
		 *
		 * @param callback - The callback function to execute when the state changes.
		 * @param level - Optional parameter to specify the listener's priority level.
		 * @returns A function that, when called, will unsubscribe the listener.
		 */
		listen: (callback: TListenerCallback<GValue>, level?: number) => () => void;
		/**
		 * Subscribes to state changes and invokes the callback immediately with the current state value.
		 *
		 * Example usage:
		 * ```js
		 * import { MY_STATE } from '../controller';
		 *
		 * const unsubscribe = MY_STATE.subscribe(value => {
		 *   console.log(value);
		 * });
		 * ```
		 *
		 * @param callback - The callback function to execute when the state changes.
		 * @param level - Optional parameter to specify the listener's priority level.
		 * @returns A function that, when called, will unsubscribe the listener.
		 */
		subscribe: (callback: TListenerCallback<GValue>, level?: number) => () => void;
		/**
		 * Triggers all registered listeners to run with the current state value.
		 */
		_notify: (process: boolean) => void;
	};
}

export interface TWithFeatures<GValue> {
	undo: { undo: () => void; _history: GValue[] };
	persist: { persist: () => Promise<boolean>; deletePersisted: () => Promise<boolean> };
}

export type TFeatures<GValue = unknown> = TCoreFeatures<GValue> & TWithFeatures<GValue>;

export type TFeatureKeys<GValue = unknown> = keyof TFeatures<GValue>;

export type TSelectFeatureObjects<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]> = {
	[K in GSelectedFeatureKeys[number]]: TFeatures<GValue>[K];
};

export type TSelectFeatures<
	GValue,
	GSelectedFeatureKeys extends TFeatureKeys<GValue>[],
	GSelectedFeatureObjects extends TSelectFeatureObjects<
		GValue,
		GSelectedFeatureKeys
	> = TSelectFeatureObjects<GValue, GSelectedFeatureKeys>
> = TUnionToIntersection<GSelectedFeatureObjects[keyof GSelectedFeatureObjects]>;

export type TEnforceFeatures<
	GFeatureKeys extends TFeatureKeys[],
	GToEnforceFeatureKeys extends TFeatureKeys[]
> = Exclude<GToEnforceFeatureKeys, GFeatureKeys> extends never
	? GFeatureKeys
	: GFeatureKeys | Exclude<GToEnforceFeatureKeys, GFeatureKeys>;

// =============================================================================
// Listener
// =============================================================================

type TListenerCallback<GValue> = (value: TReadonlyIfObject<GValue>) => Promise<void> | void;
export interface TListener<GValue> {
	callback: TListenerCallback<GValue>;
	level: number;
}

export type TListenerQueueItem<GValue = unknown> = { value: GValue } & TListener<GValue>;

// =============================================================================
// Helper
// =============================================================================

// https://fettblog.eu/typescript-union-to-intersection/
type TUnionToIntersection<T> = (T extends any ? (x: T) => any : never) extends (x: infer R) => any
	? R
	: never;

type TPrimitive = boolean | number | string;

export type TReadonlyIfObject<Value> = Value extends undefined
	? Value
	: Value extends (...args: any) => any
	? Value
	: Value extends TPrimitive
	? Value
	: Value extends object
	? Readonly<Value>
	: Value;