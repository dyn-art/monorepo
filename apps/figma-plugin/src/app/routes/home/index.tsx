import React from 'react';
import {
	Button,
	FrameIcon,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Separator
} from '@dyn/ui';

import type { TOnSelectFrameEvent } from '../../../types';
import { appHandler } from '../../app-handler';
import { Footer, Navbar } from '../../components';
import { useAppCallback } from '../../hooks';

export const plugins = [
	{
		key: 'to-dtif',
		label: 'To DTIF'
	},
	{
		key: 'node-inspector',
		label: 'Node Inspector'
	}
] as const;

export type Plugin = (typeof plugins)[number];

const Home: React.FC = () => {
	const [selectedFrames, setSelectedFrames] = React.useState<
		TOnSelectFrameEvent['args']['selected']
	>([]);
	const [selectedFrameIndex, setSelectedFrameIndex] = React.useState<number | null>(null);
	const [pluginKey, setPluginKey] = React.useState<Plugin['key']>(plugins[0].key);

	useAppCallback(appHandler, {
		type: 'plugin.message',
		key: 'on-select-frame',
		callback: async (instance, args) => {
			const selected = args.selected;
			if (selected.length > 0) {
				setSelectedFrames(selected);
				setSelectedFrameIndex(selected.length - 1);
			} else {
				setSelectedFrames([]);
				setSelectedFrameIndex(null);
			}
		}
	});

	return (
		<>
			<Navbar
				leftContent={
					<Select
						value={pluginKey}
						onValueChange={(value: Plugin['key']) => {
							setPluginKey(value);
						}}
					>
						<SelectTrigger className={'h-7 max-w-[200px] text-xs [&_svg]:h-4 [&_svg]:w-4'}>
							<span className="text-muted-foreground mr-1">Plugin: </span>
							<SelectValue placeholder="Select plugin" />
						</SelectTrigger>
						<SelectContent>
							{plugins.map((style) => (
								<SelectItem key={style.key} value={style.key} className="text-xs">
									{style.label}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				}
				rightContent={{ variant: 'user' }}
			/>

			<div>
				<div className="flex h-24 w-full flex-col items-center justify-center rounded-md border text-blue-400">
					<FrameIcon className="mb-1 h-4 w-4" />
					Select a Frame to export
				</div>
				<div className="mt-2 flex items-center justify-between">
					<Select
						defaultValue={selectedFrameIndex?.toString()}
						value={selectedFrameIndex?.toString()}
						onValueChange={(value) => {
							setSelectedFrameIndex(Number(value));
						}}
					>
						<SelectTrigger id="frame">
							<SelectValue placeholder="None selected">
								{selectedFrames[Number(selectedFrameIndex)]?.name}
							</SelectValue>
						</SelectTrigger>
						<SelectContent position="popper">
							{selectedFrames.map((plugin, index) => (
								<SelectItem key={plugin.id} value={index.toString()}>
									{plugin.name}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
					<Button className="ml-2" disabled={selectedFrameIndex == null}>
						Transform
					</Button>
				</div>
				<Separator className="my-4" />
			</div>

			<Footer leftContent={{ variant: 'version' }} rightContent={{ variant: 'settings' }} />
		</>
	);
};

export default Home;
