import type { AdditionalOperation, RulesLogic } from 'json-logic-js';
import type { TJsonFunction } from '@dyn/utils';

import type { COMP } from '../comp';

export interface TMdtifComposition {
	template: COMP.DtifComposition;
	modificationFields: TModificationField[];
}

export interface TModificationField<
	GKey extends string = string,
	GInputType extends TModificationInputType = TModificationInputType,
	GInferredKey extends GKey = GKey
> {
	key: GKey;
	inputType: GInputType;
	displayName: string;
	actions: TModificationAction<GInferredKey, TMapToDefaultType<GInputType>>[];
	// TODO: Map specified output events to this modification field,
	// because for example if the rotation changes the position modification field changes too
	// outputEventMapper?: TModificationOutputEventMapper<GInputType, any>[];
}

export interface TModificationAction<GKey extends string, GValue> {
	conditions: TModificationCondition[];
	compute?: TJsonFunction<[GKey]>;
	events: TMdtifInputEvent<GKey, GValue>[];
}

export interface TModificationCondition {
	condition: RulesLogic<AdditionalOperation>;
	notMetMessage?: string;
}

type TMakeEventModifiable<T, K extends keyof T, GKey extends string, GValue> = {
	[P in keyof T]: P extends K ? T[P] | { var: TExpandKey<GKey, GValue> } : T[P];
};

export type TMdtifInputEvent<GKey extends string, GValue> =
	| ({ type: 'UpdateFrameNode' } & TMakeEventModifiable<
			COMP.UpdateFrameNodeDtifInputEvent,
			'clipContent',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateEllipseNode' } & TMakeEventModifiable<
			COMP.UpdateEllipseNodeDtifInputEvent,
			'startingAngle' | 'endingAngle' | 'innerRadiusRatio',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateStarNode' } & TMakeEventModifiable<
			COMP.UpdateStarNodeDtifInputEvent,
			'pointCount' | 'innerRadiusRatio',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdatePolygonNode' } & TMakeEventModifiable<
			COMP.UpdatePolygonNodeDtifInputEvent,
			'pointCount',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateTextNode' } & TMakeEventModifiable<
			COMP.UpdateTextNodeDtifInputEvent,
			| 'text'
			| 'attributes'
			| 'lineWrap'
			| 'horizontalTextAlignment'
			| 'verticalTextAlignment'
			| 'sizingMode',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateSolidPaint' } & TMakeEventModifiable<
			COMP.UpdateSolidPaintDtifInputEvent,
			'color',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateImagePaint' } & TMakeEventModifiable<
			COMP.UpdateImagePaintDtifInputEvent,
			'assetId' | 'scaleMode',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateGradientPaint' } & TMakeEventModifiable<
			COMP.UpdateGradientPaintDtifInputEvent,
			'stops',
			GKey,
			GValue
	  >)
	| ({ type: 'DeleteEntity' } & COMP.DeleteEntityDtifInputEvent)
	| ({ type: 'UpdateEntityPosition' } & TMakeEventModifiable<
			COMP.UpdateEntityTransformDtifInputEvent,
			'x' | 'y' | 'rotationDeg',
			GKey,
			GValue
	  >)
	| ({ type: 'MoveEntity' } & TMakeEventModifiable<
			COMP.MoveEntityDtifInputEvent,
			'dx' | 'dy',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateEntityRotation' } & TMakeEventModifiable<
			COMP.UpdateEntityRotationDtifInputEvent,
			'rotationDeg',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateEntityVisibility' } & TMakeEventModifiable<
			COMP.UpdateEntityVisibilityInputEvent,
			'visible',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateEntityCornerRadii' } & TMakeEventModifiable<
			COMP.UpdateEntityCornerRadiiDtifInputEvent,
			'cornerRadii',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateEntityBlendMode' } & TMakeEventModifiable<
			COMP.UpdateEntityBlendModeDtifInputEvent,
			'blendMode',
			GKey,
			GValue
	  >)
	| ({ type: 'UpdateEntityOpacity' } & TMakeEventModifiable<
			COMP.UpdateEntityOpacityDtifInputEvent,
			'opacity',
			GKey,
			GValue
	  >);

export type TModificationInputType =
	| TNumberModificationInput
	| TTextModificationInput
	| TBooleanModificationInput
	| TRangeModificationInput
	| TColorModificationInput
	| TPositionModificationInput;

export interface TNumberModificationInput {
	type: 'NUMBER';
	default: number;
	max?: number;
	min?: number;
}

export interface TTextModificationInput {
	type: 'TEXT';
	default: string;
	area: boolean;
}

export interface TBooleanModificationInput {
	type: 'BOOLEAN';
	default: boolean;
}

export interface TRangeModificationInput {
	type: 'RANGE';
	default: number;
	max: number;
	min: number;
	step?: number;
}

export interface TColorModificationInput {
	type: 'COLOR';
	default: { r: number; g: number; b: number };
}

export interface TPositionModificationInput {
	type: 'POSITION';
	default: [number, number];
	max?: [number, number];
	min?: [number, number];
}

export interface TImageModificationInput {
	type: 'IMAGE';
	default?: string;
}

export type TMapToDefaultType<T> = T extends { default: infer U } ? U : never;

export type TExpandKey<GPrefix extends string, GValue> = GValue extends any[]
	? `${GPrefix}.${number}`
	: GValue extends object
		?
				| { [P in keyof GValue]: TExpandKey<`${GPrefix}.${P & string}`, GValue[P]> }[keyof GValue]
				| GPrefix
		: GPrefix;

// export interface TModificationOutputEventMapper<
// 	GKey extends string = string,
// 	GInputType extends TModificationInputType = TModificationInputType,
// 	GOutputEventType extends COMP.WatchableComponentVariant = COMP.WatchableComponentVariant,
// 	GInferredOutputEventType extends GOutputEventType = GOutputEventType
// > {
// 	type: GOutputEventType;
// 	entity: string;
// 	map: Record<
// 		GKey,
// 		TInputToOutputPathMap<
// 			TMapToDefaultType<GInputType>,
// 			TExpandKey<GInferredOutputEventType, TComponent<GInferredOutputEventType>>
// 		>
// 	>;
// }

// export type TComponent<GComponentVariant extends COMP.WatchableComponentVariant> = Omit<
// 	Extract<COMP.ComponentChange, { type: GComponentVariant }>,
// 	'type'
// >;

// export type TInputToOutputPathMap<GInput, GOutputPath> = GInput extends any[]
// 	? GOutputPath[]
// 	: GInput extends object
// 		? { [P in keyof GInput]?: GOutputPath }[keyof GInput]
// 		: GOutputPath;

export type TFieldData<GKey extends string, GInputType extends TModificationInputType> = {
	[key in GKey]: TMapToDefaultType<GInputType>;
};
