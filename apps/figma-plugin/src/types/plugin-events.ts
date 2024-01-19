import type { TPluginMessageEvent } from '@dyn/figma-handler/app';
import type { COMP, TTransformStatusUpdate } from '@dyn/figma-to-dtif';

export interface TOnSelectFrameEvent extends TPluginMessageEvent {
	key: 'on-select-frame';
	args: {
		selected: Pick<FrameNode | ComponentNode | InstanceNode, 'name' | 'id'>[];
	};
}

export interface TOnSelectNodeEvent extends TPluginMessageEvent {
	key: 'on-select-node';
	args: { selected: Pick<SceneNode, 'name' | 'id'>[] };
}

export interface TOnSelectNodePropertiesEvent extends TPluginMessageEvent {
	key: 'on-select-node-properties';
	args: { selected: SceneNode[] };
}

export interface TOnChangeSelectedNodePropertiesEvent extends TPluginMessageEvent {
	key: 'on-change-selected-node-properties';
	args: { changed: SceneNode };
}

export interface TOnTransformStatusUpdateEvent extends TPluginMessageEvent {
	key: 'on-transform-status-update';
	args: { status: TTransformStatusUpdate };
}

export interface TIntermediateFormatExportResultEvent extends TPluginMessageEvent {
	key: 'intermediate-format-export-result';
	args:
		| {
				type: 'error';
				message: string;
		  }
		| {
				type: 'success';
				content: COMP.DTIFComposition;
		  };
}

export type TPluginMessageEvents =
	| TOnSelectFrameEvent
	| TOnSelectNodeEvent
	| TOnSelectNodePropertiesEvent
	| TOnChangeSelectedNodePropertiesEvent
	| TOnTransformStatusUpdateEvent
	| TIntermediateFormatExportResultEvent;
