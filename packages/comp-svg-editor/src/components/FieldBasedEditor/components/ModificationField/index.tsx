import React from 'react';
import type {
	TBooleanModificationInput,
	TColorModificationInput,
	TModificationField,
	TNumberModificationInput,
	TPositionModificationInput,
	TRangeModificationInput,
	TTextModificationInput
} from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

import { BooleanInput } from './BooleanInput';
import { ColorInput } from './ColorInput';
import { NumberInput } from './NumberInput';
import { PositionInput } from './PositionInput';
import { RangeInput } from './RangeInput';
import { TextInput } from './TextInput';

export * from './BooleanInput';
export * from './NumberInput';
export * from './PositionInput';
export * from './RangeInput';
export * from './TextInput';

export const ModificationField: React.FC<TProps> = (props) => {
	const { field, composition } = props;

	switch (field.inputType.type) {
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
		default:
			return <p>Coming Soon</p>;
	}
};

interface TProps {
	field: TModificationField;
	composition: Composition;
}
