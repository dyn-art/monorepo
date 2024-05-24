import React from 'react';
import type {
	TBooleanModificationInput,
	TColorModificationInput,
	TDateTimeModificationInput,
	TModificationField,
	TNumberModificationInput,
	TPaintModificationInput,
	TPositionModificationInput,
	TRangeModificationInput,
	TTextModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

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

export const ModificationField: React.FC<TProps> = (props) => {
	const { field, composition } = props;

	switch (field.inputVariant.type) {
		case 'POSITION':
			return (
				<PositionInput
					composition={composition}
					field={field as TModificationField<string, TPositionModificationInput>}
					key={field.key}
				/>
			);
		case 'NUMBER':
			return (
				<NumberInput
					composition={composition}
					field={field as TModificationField<string, TNumberModificationInput>}
					key={field.key}
				/>
			);
		case 'TEXT':
			return (
				<TextInput
					composition={composition}
					field={field as TModificationField<string, TTextModificationInput>}
					key={field.key}
				/>
			);
		case 'BOOLEAN':
			return (
				<BooleanInput
					composition={composition}
					field={field as TModificationField<string, TBooleanModificationInput>}
					key={field.key}
				/>
			);
		case 'RANGE':
			return (
				<RangeInput
					composition={composition}
					field={field as TModificationField<string, TRangeModificationInput>}
					key={field.key}
				/>
			);
		case 'COLOR':
			return (
				<ColorInput
					composition={composition}
					field={field as TModificationField<string, TColorModificationInput>}
					key={field.key}
				/>
			);
		case 'PAINT':
			return (
				<PaintInput
					composition={composition}
					field={field as TModificationField<string, TPaintModificationInput>}
					key={field.key}
				/>
			);
		case 'DATETIME':
			return (
				<DateTimeInput
					composition={composition}
					field={field as TModificationField<string, TDateTimeModificationInput>}
					key={field.key}
				/>
			);
	}
};

interface TProps {
	field: TModificationField;
	composition: Composition;
}
