import * as jsonLogic from 'json-logic-js';
import { describe, expect, it } from 'vitest';

import {
	applyModifications,
	type TResolvedFieldAction,
	type TUnresolvedFieldAction
} from './apply-modifications';
import { createModificationField } from './create-modification-field';

describe('processField function', () => {
	it('json logic playground', () => {
		const result = jsonLogic.apply({ '>': [{ var: 'pos.1' }, 0] }, { pos: [-10, 10] });

		expect(result).toBeTruthy();
	});

	it('applies modifications to a field correctly when all conditions are met', () => {
		const field = createModificationField({
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
					compute: { args: ['moveX'], body: 'return moveX + 10' },
					events: [{ type: 'MoveEntity', entity: 'n1', dx: { var: 'moveX' }, dy: 0 }]
				}
			]
		});

		const results = applyModifications(field, { moveX: 30 });
		const firstResult = results[0] as TResolvedFieldAction;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeTruthy();
		expect(firstResult.events[0]).toEqual({ type: 'EntityMoved', entity: 'n1', dx: 40, dy: 0 });
	});

	it('handles not met conditions by returning the appropriate messages', () => {
		const field = createModificationField({
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
					events: [{ type: 'MoveEntity', entity: 'n1', dx: { var: 'moveX' }, dy: 0 }]
				}
			]
		});

		const results = applyModifications(field, { moveX: -10 });
		const firstResult = results[0] as TUnresolvedFieldAction;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeFalsy();
		expect(firstResult.notMetConditions[0]).toEqual({
			index: 0,
			message: "'moveX' can not be negative!"
		});
	});

	it('applies modifications to an array field correctly when all conditions are met', () => {
		const field = createModificationField({
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
							type: 'UpdateEntityPosition',
							entity: 'n1',
							x: { var: 'pos.0' },
							y: { var: 'pos.1' }
						}
					]
				}
			]
		});

		const results = applyModifications(field, { pos: [20, 10] });
		const firstResult = results[0] as TResolvedFieldAction;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeTruthy();
		expect(firstResult.events[0]).toEqual({
			type: 'EntitySetPosition',
			entity: 'n1',
			x: 20,
			y: 10
		});
	});

	it('handles not met conditions by returning the appropriate messages for an array field', () => {
		const field = createModificationField({
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
							type: 'UpdateEntityPosition',
							entity: 'n1',
							x: { var: 'pos.0' },
							y: { var: 'pos.1' }
						}
					]
				}
			]
		});

		const results = applyModifications(field, { pos: [-10, 10] });
		const firstResult = results[0] as TUnresolvedFieldAction;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeFalsy();
		expect(firstResult.notMetConditions[0]).toEqual({
			index: 0,
			message: 'x can not be negative!'
		});
	});

	it('applies modifications to a object field correctly when all conditions are met', () => {
		const field = createModificationField({
			key: 'color',
			displayName: 'Set Position',
			inputType: { type: 'COLOR', default: { r: 0, g: 0, b: 0 } },
			actions: [
				{
					conditions: [
						{
							condition: {
								and: [{ '>': [{ var: 'color.r' }, 0] }, { '<=': [{ var: 'color.r' }, 255] }]
							},
							notMetMessage: 'Red is out of spectrum!'
						},
						{
							condition: {
								and: [{ '>': [{ var: 'color.g' }, 0] }, { '<=': [{ var: 'color.g' }, 255] }]
							},
							notMetMessage: 'Green is out of spectrum!'
						},
						{
							condition: {
								and: [{ '>': [{ var: 'color.b' }, 0] }, { '<=': [{ var: 'color.b' }, 255] }]
							},
							notMetMessage: 'Blue is out of spectrum!'
						}
					],
					events: [
						{
							type: 'UpdateEntityPosition',
							entity: 'n1',
							x: { var: 'color.r' },
							y: { var: 'color.g' }
						},
						{
							type: 'UpdateEntityPosition',
							entity: 'n1',
							x: { var: 'color.g' },
							y: { var: 'color.b' }
						}
					]
				}
			]
		});

		const results = applyModifications(field, { color: { r: 10, g: 20, b: 30 } });
		const firstResult = results[0] as TResolvedFieldAction;

		expect(firstResult).not.toBeNull();
		expect(firstResult.resolved).toBeTruthy();
		expect(firstResult.events[0]).toEqual({
			type: 'EntitySetPosition',
			entity: 'n1',
			x: 10,
			y: 20
		});
		expect(firstResult.events[1]).toEqual({
			type: 'EntitySetPosition',
			entity: 'n1',
			x: 20,
			y: 30
		});
	});
});
