import type { TListener, TListenerQueueItem, TReadonlyIfObject, TState } from './types';

const listenerQueue: TListenerQueueItem[] = [];

export function createState<GValue>(value: GValue): TState<GValue, ['get', 'set', 'listen']> {
	const state: TState<GValue, ['get', 'set', 'listen']> = {
		_listeners: [],
		_value: value,
		get() {
			return this._value;
		},
		set(newValue) {
			if (this._value !== newValue) {
				this._value = newValue;
				this.notify();
			}
		},
		listen(callback, level) {
			const listener: TListener<GValue> = {
				callback,
				level: level ?? 0
			};
			this._listeners.push(listener);
			return () => {
				this._listeners.filter((l) => l !== listener);
			};
		},
		subscribe(callback, level) {
			const unbind = this.listen(callback, level);
			callback(this._value as TReadonlyIfObject<GValue>);
			return unbind;
		},
		notify() {
			// Add current state's listeners to the queue
			this._listeners.forEach((listener) => {
				const queueItem: TListenerQueueItem<GValue> = {
					value: this._value,
					...listener
				};
				listenerQueue.push(queueItem as TListenerQueueItem);
			});

			// Defer processing using setTimeout
			setTimeout(() => {
				// Drain the queue
				const queueToProcess = listenerQueue.slice();

				// Sort the drained listeners and execute the callbacks
				queueToProcess
					.sort((a, b) => a.level - b.level)
					.forEach((queueItem) => {
						queueItem.callback(queueItem.value);
					});
			});
		}
	};

	return state;
}
