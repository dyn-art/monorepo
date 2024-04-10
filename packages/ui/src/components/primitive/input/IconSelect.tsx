'use client';

import { CaretSortIcon } from '@radix-ui/react-icons';
import * as SelectPrimitive from '@radix-ui/react-select';
import React from 'react';

import { Select, SelectContent, SelectItem, SelectValue } from './Select';

export const IconSelect = <GItemMap extends Record<string, TIconSelectItem>>(
	props: TIconSelectProps<GItemMap>
): React.ReactNode => {
	const { items, value, onValueChange, ...selectProps } = props;

	return (
		<Select
			{...selectProps}
			onValueChange={(val) => {
				onValueChange(val as keyof typeof items);
			}}
			value={value as string}
		>
			<div className="flex h-9 w-[50px] items-center justify-between gap-1 whitespace-nowrap rounded-md bg-transparent px-2 py-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1">
				<SelectValue aria-label={value as string}>{(items[value] as any).icon}</SelectValue>
				<SelectPrimitive.Trigger>
					<SelectPrimitive.Icon asChild>
						<CaretSortIcon className="h-4 w-4 opacity-50" />
					</SelectPrimitive.Icon>
				</SelectPrimitive.Trigger>
			</div>

			<SelectContent>
				{Object.entries(items).map(([key, _value]) => {
					return (
						<SelectItem key={key} value={key}>
							<div className="flex items-center gap-2">
								{_value.icon}
								{_value.text}
							</div>
						</SelectItem>
					);
				})}
			</SelectContent>
		</Select>
	);
};

export interface TIconSelectProps<GItemMap extends Record<string, TIconSelectItem>>
	extends Omit<Omit<React.ComponentPropsWithoutRef<typeof Select>, 'onValueChange'>, 'value'> {
	items: GItemMap;
	onValueChange: (value: keyof GItemMap) => void;
	value: keyof GItemMap;
}

export interface TIconSelectItem {
	icon: React.ReactElement;
	text: string;
}
