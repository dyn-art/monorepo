import * as jsonLogic from 'json-logic-js';
import { describe, expect, it } from 'vitest';

import { createField } from './create-field';
import { processField, type TResolvedField, type TUnresolvedField } from './process-field';

describe('processField function', () => {
	it('json logic playground', () => {
		const result = jsonLogic.apply({ '>': [{ var: 'pos.1' }, 0] }, { pos: [-10, 10] });

		expect(result).toBeTruthy();
	});

	it('processes a field correctly when all conditions are met', () => {
		const field = createField({
			key: 'moveX',
			displayName: 'Move X',
			inputType: { type: 'NUMBER', default: 0 },
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

		const results = processField(field, { moveX: 30 });
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

		const results = processField(field, { moveX: -10 });
		const firstResult = results[0] as TUnresolvedField;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeFalsy();
		expect(firstResult.notMetConditions[0]).toEqual({
			index: 0,
			message: "'moveX' can not be negative!"
		});
	});

	it('processes a array field correctly when all conditions are met', () => {
		const field = createField({
			key: 'pos',
			displayName: 'Set Position',
			inputType: { type: 'POSITION', default: [0, 0] },
			actions: [
				{
					conditions: [
						{
							condition: { '>': [{ var: 'pos.0' }, 0] },
							notMetMessage: 'x can not be negative!'
						},
						{
							condition: { '>': [{ var: 'pos.1' }, 0] },
							notMetMessage: 'y can not be negative!'
						}
					],
					events: [
						{
							type: 'EditableEntitySetPosition',
							entity: 'n1',
							x: { var: 'pos.0' },
							y: { var: 'pos.1' }
						}
					]
				}
			]
		});

		const results = processField(field, { pos: [20, 10] });
		const firstResult = results[0] as TResolvedField;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeTruthy();
		expect(firstResult.events[0]).toEqual({
			type: 'EntitySetPosition',
			entity: 'n1',
			x: 20,
			y: 10
		});
	});

	it('handles not met conditions by returning the appropriate messages for array field', () => {
		const field = createField({
			key: 'pos',
			displayName: 'Set Position',
			inputType: { type: 'POSITION', default: [0, 0] },
			actions: [
				{
					conditions: [
						{
							condition: { '>': [{ var: 'pos.0' }, 0] },
							notMetMessage: 'x can not be negative!'
						},
						{
							condition: { '>': [{ var: 'pos.1' }, 0] },
							notMetMessage: 'y can not be negative!'
						}
					],
					events: [
						{
							type: 'EditableEntitySetPosition',
							entity: 'n1',
							x: { var: 'pos.0' },
							y: { var: 'pos.1' }
						}
					]
				}
			]
		});

		const results = processField(field, { pos: [-10, 10] });
		const firstResult = results[0] as TUnresolvedField;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeFalsy();
		expect(firstResult.notMetConditions[0]).toEqual({
			index: 0,
			message: 'x can not be negative!'
		});
	});
});
