import type { TPluginMessageEvent } from '@dyn/figma-handler/app';

export interface TOnSelectFrameEvent extends TPluginMessageEvent {
	key: 'on-select-frame';
	args: {
		selected: Pick<FrameNode | ComponentNode | InstanceNode, 'name' | 'id'>[];
	};
}

export type TPluginMessageEvents = TOnSelectFrameEvent;
