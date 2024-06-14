import type { TFromPluginMessageEvent } from 'figma-connect/app';
import type { CNV, TTransformStatusUpdate } from '@dyn/figma-to-dtif';

export interface TOnSelectFrameEvent extends TFromPluginMessageEvent {
	key: 'on-select-frame';
	args: {
		selected: Pick<FrameNode | ComponentNode | InstanceNode, 'name' | 'id'>[];
	};
}

export interface TOnSelectNodeEvent extends TFromPluginMessageEvent {
	key: 'on-select-node';
	args: { selected: Pick<SceneNode, 'name' | 'id'>[] };
}

export interface TOnSelectNodePropertiesEvent extends TFromPluginMessageEvent {
	key: 'on-select-node-properties';
	args: { selected: SceneNode[] };
}

export interface TOnChangeSelectedNodePropertiesEvent extends TFromPluginMessageEvent {
	key: 'on-change-selected-node-properties';
	args: { changed: SceneNode };
}

export interface TOnTransformStatusUpdateEvent extends TFromPluginMessageEvent {
	key: 'on-transform-status-update';
	args:
		| { type: 'Start' }
		| { type: 'Transform'; status: TTransformStatusUpdate }
		| { type: 'Transmit' }
		| { type: 'End' };
}

export interface TIntermediateFormatExportResultEvent extends TFromPluginMessageEvent {
	key: 'intermediate-format-export-result';
	args:
		| {
				type: 'error';
				message: string;
		  }
		| {
				type: 'success';
				content: CNV.DtifCanvas;
		  };
}

export type TFromPluginMessageEvents =
	| TOnSelectFrameEvent
	| TOnSelectNodeEvent
	| TOnSelectNodePropertiesEvent
	| TOnChangeSelectedNodePropertiesEvent
	| TOnTransformStatusUpdateEvent
	| TIntermediateFormatExportResultEvent;
