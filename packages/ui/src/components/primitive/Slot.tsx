import * as SlotPrimitive from '@radix-ui/react-slot';

// Before React 19 accessing `element.props.ref` will throw a warning and suggest using `element.ref`
// After React 19 accessing `element.ref` does the opposite.
// https://github.com/facebook/react/pull/28348
//
// Access the ref using the method that doesn't yield a warning.
//
// Based on: https://github.com/radix-ui/primitives/blob/main/packages/react/slot/src/Slot.tsx#L131
export function getElementRef(element: React.ReactElement) {
	// React <=18 in DEV
	let getter = Object.getOwnPropertyDescriptor(element.props, 'ref')?.get;
	let mayWarn = getter && 'isReactWarning' in getter && getter.isReactWarning;
	if (mayWarn) {
		return (element as any).ref;
	}

	// React 19 in DEV
	getter = Object.getOwnPropertyDescriptor(element, 'ref')?.get;
	mayWarn = getter && 'isReactWarning' in getter && getter.isReactWarning;
	if (mayWarn) {
		return element.props.ref;
	}

	// Not DEV
	return element.props.ref || (element as any).ref;
}

const Slot = SlotPrimitive.Slot;
const Slottable = SlotPrimitive.Slottable;

export * from '@radix-ui/react-compose-refs';
export { Slot, Slottable };
