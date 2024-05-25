import { apply } from 'json-logic-js';
import { deepReplaceVar, type TJsonFunction } from '@dyn/utils';

import type { COMP } from '../comp';
import type {
	TFieldData as TFieldModifications,
	TMapToReturnType,
	TMdtifInputEvent,
	TModificationField,
	TModificationInputVariant
} from './types';

export async function applyModifications<
	GKey extends string,
	GInputVariant extends TModificationInputVariant
>(
	field: TModificationField<GKey, GInputVariant>,
	modifications: TFieldModifications<GKey, GInputVariant>,
	runJsonFunction?: <T extends string[]>(
		jsonFunction: TJsonFunction<T>,
		args: unknown[]
	) => Promise<any>
): Promise<TProcessedFieldAction[]> {
	const { actions } = field;
	const processedActions: TProcessedFieldAction[] = [];

	for (const action of actions) {
		const { conditions, events, compute } = action;
		const actionModifications = { ...modifications };

		if (compute != null && runJsonFunction != null) {
			actionModifications[
				compute.resultName != null ? (compute.resultName as GKey) : compute.args[0]
			] = await runJsonFunction(compute, [actionModifications[compute.args[0]]]);
		}

		// Check whether data matches conditions for action
		const notMetConditions: TNotMetCondition[] = [];
		for (const [index, condition] of conditions.entries()) {
			const metCondition = apply(condition.condition, actionModifications);
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
					toDtifInputEvent<GKey, TMapToReturnType<GInputVariant>>(event, actionModifications)
				)
			});
		}
	}

	return processedActions;
}

function toDtifInputEvent<GKey extends string, GValue>(
	event: TMdtifInputEvent<GKey, GValue>,
	data: Record<string, any>
): COMP.CoreInputEvent {
	const result = deepReplaceVar(event, data);
	return result as COMP.CoreInputEvent;
}

export type TProcessedFieldAction = TResolvedFieldAction | TUnresolvedFieldAction;

export interface TResolvedFieldAction {
	resolved: true;
	events: COMP.CoreInputEvent[];
}

export interface TUnresolvedFieldAction {
	resolved: false;
	notMetConditions: TNotMetCondition[];
}

interface TNotMetCondition {
	index: number;
	message?: string;
}
