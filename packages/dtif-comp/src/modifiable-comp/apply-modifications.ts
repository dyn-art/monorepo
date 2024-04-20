import { apply } from 'json-logic-js';
import { deepReplaceVar } from '@dyn/utils';

import type { COMP } from '../comp';
import type {
	ModifiableDtifInputEvent,
	TFieldData as TFieldModifications,
	TMapToDefaultType,
	TModificationField,
	TModificationInputType
} from './types';

export function applyModifications<GKey extends string, GInputType extends TModificationInputType>(
	field: TModificationField<GKey, GInputType>,
	modifications: TFieldModifications<GKey, GInputType>
): TProcessedFieldResult[] {
	const { actions } = field;
	const results: TProcessedFieldResult[] = [];

	for (const action of actions) {
		const { conditions, events } = action;

		// Check whether data matches conditions for action
		const notMetConditions: TNotMetCondition[] = [];
		for (const [index, condition] of conditions.entries()) {
			const metCondition = apply(condition.condition, modifications);
			if (!metCondition) {
				notMetConditions.push({ index, message: condition.notMetMessage });
			}
		}

		if (notMetConditions.length > 0) {
			results.push({ resolved: false, notMetConditions });
		} else {
			results.push({
				resolved: true,
				events: events.map((event) =>
					prepareEvent<GKey, TMapToDefaultType<GInputType>>(event, modifications)
				)
			});
		}
	}

	return results;
}

// TODO: Make safer?
function prepareEvent<GKey extends string, GValue>(
	event: COMP.DtifInputEvent | ModifiableDtifInputEvent<GKey, GValue>,
	data: Record<string, any>
): COMP.DtifInputEvent {
	if (event.type.startsWith('Editable')) {
		const result = deepReplaceVar(event, data);
		result.type = result.type.replace('Editable', '') as any;
		return result as COMP.DtifInputEvent;
	}
	return event as COMP.DtifInputEvent;
}

export type TProcessedFieldResult = TResolvedField | TUnresolvedField;

export interface TResolvedField {
	resolved: true;
	events: COMP.DtifInputEvent[];
}

export interface TUnresolvedField {
	resolved: false;
	notMetConditions: TNotMetCondition[];
}

interface TNotMetCondition {
	index: number;
	message?: string;
}
