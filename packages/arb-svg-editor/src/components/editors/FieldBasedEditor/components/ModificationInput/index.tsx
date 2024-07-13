import React from 'react';
import type {
	TBooleanModificationInput,
	TColorModificationInput,
	TDateTimeModificationInput,
	TModificationScript,
	TNumberModificationInput,
	TPaintModificationInput,
	TPositionModificationInput,
	TRangeModificationInput,
	TTextModificationInput
} from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

import { BooleanInput } from './BooleanInput';
import { ColorInput } from './ColorInput';
import { DateTimeInput } from './DateTimeInput';
import { NumberInput } from './NumberInput';
import { PaintInput } from './PaintInput';
import { PositionInput } from './PositionInput';
import { RangeInput } from './RangeInput';
import { TextInput } from './TextInput';

export * from './BooleanInput';
export * from './ColorInput';
export * from './DateTimeInput';
export * from './NumberInput';
export * from './PositionInput';
export * from './RangeInput';
export * from './TextInput';

export const ModificationInput: React.FC<TProps> = (props) => {
	const { script, artboard } = props;

	switch (script.inputVariant.type) {
		case 'POSITION':
			return (
				<PositionInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TPositionModificationInput>}
				/>
			);
		case 'NUMBER':
			return (
				<NumberInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TNumberModificationInput>}
				/>
			);
		case 'TEXT':
			return (
				<TextInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TTextModificationInput>}
				/>
			);
		case 'BOOLEAN':
			return (
				<BooleanInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TBooleanModificationInput>}
				/>
			);
		case 'RANGE':
			return (
				<RangeInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TRangeModificationInput>}
				/>
			);
		case 'COLOR':
			return (
				<ColorInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TColorModificationInput>}
				/>
			);
		case 'PAINT':
			return (
				<PaintInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TPaintModificationInput>}
				/>
			);
		case 'DATETIME':
			return (
				<DateTimeInput
					artboard={artboard}
					key={script.id}
					script={script as TModificationScript<TDateTimeModificationInput>}
				/>
			);
	}
};

interface TProps {
	script: TModificationScript;
	artboard: Artboard;
}
