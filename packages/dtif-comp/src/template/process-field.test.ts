import { describe, expect, it } from 'vitest';

import { createField } from './create-field';
import { processField, type TResolvedField, type TUnresolvedField } from './process-field';

describe('processField function', () => {
	it('processes a field correctly when all conditions are met', () => {
		const field = createField({
			key: 'moveX',
			displayName: 'Move X',
			inputType: { type: 'NUMBER', default: 10 },
			actions: [
				{
					conditions: [
						{
							condition: { '>': [{ var: 'moveX' }, 0] },
							notMetMessage: "'moveX' can not be negative!"
						}
					],
					events: [{ type: 'EditableEntityMoved', entity: 'n1', dx: { var: 'moveX' }, dy: 0 }]
				}
			]
		});

		const value = { moveX: 30 };
		const results = processField(field, value);
		const firstResult = results[0] as TResolvedField;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeTruthy();
		expect(firstResult.events[0]).toEqual({ type: 'EntityMoved', entity: 'n1', dx: 30, dy: 0 });
	});

	it('handles not met conditions by returning the appropriate messages', () => {
		const field = createField({
			key: 'moveX',
			displayName: 'Move X',
			inputType: { type: 'NUMBER', default: 10 },
			actions: [
				{
					conditions: [
						{
							condition: { '>': [{ var: 'moveX' }, 0] },
							notMetMessage: "'moveX' can not be negative!"
						}
					],
					events: [{ type: 'EditableEntityMoved', entity: 'n1', dx: { var: 'moveX' }, dy: 0 }]
				}
			]
		});

		const value = { moveX: -10 };
		const results = processField(field, value);
		const firstResult = results[0] as TUnresolvedField;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeFalsy();
		expect(firstResult.notMetConditions[0]).toEqual({
			index: 0,
			message: "'moveX' can not be negative!"
		});
	});
});
