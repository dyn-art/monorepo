import { apply } from 'json-logic-js';
import { deepReplaceVar } from '@dyn/utils';

import type { COMP } from '../comp';
import type {
	TFieldData as TFieldModifications,
	TMapToDefaultType,
	TMdtifInputEvent,
	TMdtifInputEventType,
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
	event: COMP.DtifInputEvent | TMdtifInputEvent<GKey, GValue>,
	data: Record<string, any>
): COMP.DtifInputEvent {
	if (isMdtifInputEvent(event.type)) {
		const result = deepReplaceVar(event, data);
		result.type = result.type.replace('Editable', '') as COMP.DtifInputEvent['type'];
		return result as COMP.DtifInputEvent;
	}
	return event as COMP.DtifInputEvent;
}

function isMdtifInputEvent(value: unknown): value is TMdtifInputEventType {
	return typeof value === 'string' && value.startsWith('Editable');
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
