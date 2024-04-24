import { apply } from 'json-logic-js';
import { deepReplaceVar } from '@dyn/utils';

import type { COMP } from '../comp';
import type {
	TFieldData as TFieldModifications,
	TMapToDefaultType,
	TMdtifInputEvent,
	TModificationField,
	TModificationInputType
} from './types';

export function applyModifications<GKey extends string, GInputType extends TModificationInputType>(
	field: TModificationField<GKey, GInputType>,
	modifications: TFieldModifications<GKey, GInputType>
): TProcessedFieldAction[] {
	const { actions } = field;
	const processedActions: TProcessedFieldAction[] = [];

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
			processedActions.push({ resolved: false, notMetConditions });
		} else {
			processedActions.push({
				resolved: true,
				events: events.map((event) =>
					toDtifInputEvent<GKey, TMapToDefaultType<GInputType>>(event, modifications)
				)
			});
		}
	}

	return processedActions;
}

function toDtifInputEvent<GKey extends string, GValue>(
	event: TMdtifInputEvent<GKey, GValue>,
	data: Record<string, any>
): COMP.DtifInputEvent {
	const result = deepReplaceVar(event, data);
	return result as COMP.DtifInputEvent;
}

export type TProcessedFieldAction = TResolvedFieldAction | TUnresolvedFieldAction;

export interface TResolvedFieldAction {
	resolved: true;
	events: COMP.DtifInputEvent[];
}

export interface TUnresolvedFieldAction {
	resolved: false;
	notMetConditions: TNotMetCondition[];
}

interface TNotMetCondition {
	index: number;
	message?: string;
}
