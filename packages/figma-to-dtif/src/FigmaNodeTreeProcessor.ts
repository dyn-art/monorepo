import { MD5 } from 'object-hash';
import type { COMP } from '@dyn/comp-dtif';
import { ContinuousId, notEmpty, type TContinuousId } from '@dyn/utils';

import { UnsupportedFigmaNodeException } from './exceptions';
import type {
	TFigmaNodeWithChildren,
	TFigmaNodeWithFills,
	TFigmaNodeWithStrokes,
	TFigmaShapeNode
} from './types';
import {
	dropMixed,
	isFigmaComponentNode,
	isFigmaFrameNode,
	isFigmaGroupNode,
	isFigmaInstanceNode,
	isFigmaSceneNode,
	isFigmaShapeNode,
	isFigmaTextNode
} from './utils';

export class FigmaNodeTreeProcessor {
	private _root: FrameNode;
	private _toTransformNodes: TToTransformNode[] = [];

	private _toTransformPaints: TToTransformPaint[] = [];
	private _toTransformPaintsHashmap = new Map<string, TToTransformHashmapItem>();

	private _toTransformAssets: TToTransformAsset[] = [];
	private _toTransformAssetsHashmap = new Map<string, TToTransformHashmapItem>();

	constructor(root: FrameNode) {
		this._root = root;
	}

	// Entry method to start processing the node tree
	public processNodeTree(): {
		rootId: TContinuousId;
		toTransformNodes: TToTransformNode[];
		toTransformPaints: TToTransformPaint[];
		toTransformAssets: TToTransformAsset[];
	} {
		const rootId = this.walk(this._root, true);

		return {
			rootId,
			toTransformNodes: this._toTransformNodes,
			toTransformPaints: this._toTransformPaints,
			toTransformAssets: this._toTransformAssets
		};
	}

	// Recursive method to walk through each node
	private walk(node: SceneNode, isRoot = false): TContinuousId {
		const nodeId = ContinuousId.nextId();

		if (isFigmaFrameNode(node) || isFigmaComponentNode(node) || isFigmaInstanceNode(node)) {
			this._toTransformNodes.push({
				type: 'Frame',
				id: nodeId,
				node,
				childrenIds: this.processChildren(node),
				fills: this.processFills(node),
				strokes: this.processStrokes(node),
				isRoot
			});
		} else if (isFigmaGroupNode(node)) {
			this._toTransformNodes.push({
				type: 'Group',
				id: nodeId,
				node,
				childrenIds: this.processChildren(node)
			});
		} else if (isFigmaTextNode(node)) {
			this._toTransformNodes.push({
				type: 'Text',
				id: nodeId,
				node,
				segments: this.processTextSegments(node),
				strokes: this.processStrokes(node),
				fills: this.processFills(node)
			});
		} else if (isFigmaShapeNode(node)) {
			this._toTransformNodes.push({
				type: 'Shape',
				id: nodeId,
				node,
				fills: this.processFills(node),
				strokes: this.processStrokes(node)
			});
		} else if (isFigmaSceneNode(node)) {
			this._toTransformNodes.push({
				type: 'Uncategorized',
				id: nodeId,
				node
			});
		} else {
			throw new UnsupportedFigmaNodeException(node);
		}

		return nodeId;
	}

	// Processes children of a node
	private processChildren(node: TFigmaNodeWithChildren): TContinuousId[] {
		// Reverse so that the most top node is the first item in the array
		return node.children.map((child) => this.walk(child)).reverse();
	}

	// Processes fills of a node
	private processFills(node: TFigmaNodeWithFills): TToTransformFill[] {
		const fills = dropMixed(node, 'fills');
		return fills
			.map((fill) => {
				let paint: TToTransformPaintVariant;
				switch (fill.type) {
					case 'SOLID':
					case 'GRADIENT_LINEAR':
					case 'GRADIENT_RADIAL':
					case 'GRADIENT_ANGULAR':
					case 'GRADIENT_DIAMOND':
						paint = fill;
						break;
					case 'IMAGE':
						paint = {
							...fill,
							assetId: this.getOrGenerateId(
								this._toTransformAssetsHashmap,
								this._toTransformAssets,
								{
									nodeIds: [node.id],
									asset: { type: 'Image', hash: fill.imageHash }
								}
							)
						};
						break;
					case 'VIDEO':
						return null;
				}

				return {
					paintId: this.getOrGenerateId(this._toTransformPaintsHashmap, this._toTransformPaints, {
						nodeIds: [node.id],
						paint
					}),
					visible: fill.visible ?? true,
					opacity: fill.opacity ?? 1,
					blendMode: fill.blendMode ?? 'PASS_THROUGH'
				};
			})
			.filter(notEmpty)
			.reverse(); // Reverse so that the most top style is the first item in the array
	}

	// Processes strokes of a node
	private processStrokes(node: TFigmaNodeWithStrokes): TToTransformStroke[] {
		const strokes = dropMixed(node, 'strokes');
		return strokes
			.map((stroke) => {
				let paint: TToTransformPaintVariant;
				switch (stroke.type) {
					case 'SOLID':
					case 'GRADIENT_LINEAR':
					case 'GRADIENT_RADIAL':
					case 'GRADIENT_ANGULAR':
					case 'GRADIENT_DIAMOND':
						paint = stroke;
						break;
					case 'IMAGE':
						paint = {
							...stroke,
							assetId: this.getOrGenerateId(
								this._toTransformAssetsHashmap,
								this._toTransformAssets,
								{
									nodeIds: [node.id],
									asset: { type: 'Image', hash: stroke.imageHash }
								}
							)
						};
						break;
					case 'VIDEO':
						return null;
				}

				return {
					paintId: this.getOrGenerateId(this._toTransformPaintsHashmap, this._toTransformPaints, {
						nodeIds: [node.id],
						paint
					}),
					visible: stroke.visible ?? true,
					opacity: stroke.opacity ?? 1,
					blendMode: stroke.blendMode ?? 'PASS_THROUGH',
					width: dropMixed(node, 'strokeWeight')
				};
			})
			.filter(notEmpty);
	}

	// Processes text segments of a text node
	private processTextSegments(node: TextNode): TTextNodeSegment[] {
		const segments = node.getStyledTextSegments([
			'fontSize',
			'fontName',
			'fontWeight',
			'fontSize',
			'letterSpacing',
			'lineHeight'
		]);
		return segments.map((segment) => ({
			...segment,
			fontId: this.getOrGenerateId(this._toTransformAssetsHashmap, this._toTransformAssets, {
				nodeIds: [node.id],
				asset: { type: 'Font', metadata: this.extractFontMetadata(segment) }
			}),
			fontMetadata: this.extractFontMetadata(segment)
		}));
	}

	// Helper to extract font metadata from a node
	private extractFontMetadata(
		segment: Omit<Omit<TTextNodeSegment, 'fontId'>, 'fontMetadata'>
	): COMP.FontMetadata {
		return {
			family: segment.fontName.family,
			weight: segment.fontWeight,
			style: segment.fontName.style.toLowerCase().includes('italic') ? 'Italic' : 'Normal'
		};
	}

	// Generates a unique ID for an item or retrieves an existing one
	private getOrGenerateId<GValue extends { id: TContinuousId; nodeIds: SceneNode['id'][] }>(
		hashMap: Map<string, TToTransformHashmapItem>,
		toTransformArray: GValue[],
		value: Omit<GValue, 'id'>
	): TContinuousId {
		const { nodeIds, ...toHash } = value;
		const hash = MD5(toHash);
		let item = hashMap.get(hash);

		if (item == null) {
			item = { id: ContinuousId.nextId(), index: toTransformArray.length };
			toTransformArray.push({ ...value, id: item.id } as unknown as GValue);
			hashMap.set(hash, item);
		} else {
			toTransformArray[item.index]?.nodeIds.push(...value.nodeIds);
		}

		return item.id;
	}
}

interface TToTransformBaseNode {
	type: 'Text' | 'Frame' | 'Group' | 'Shape' | 'Uncategorized';
	id: TContinuousId;
	node: SceneNode;
}

export interface TToTransformTextNode extends TToTransformBaseNode {
	type: 'Text';
	node: TextNode;
	segments: TTextNodeSegment[];
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}

export type TTextNodeSegment = Pick<
	StyledTextSegment,
	| 'fontSize'
	| 'fontName'
	| 'fontWeight'
	| 'letterSpacing'
	| 'lineHeight'
	| 'characters'
	| 'start'
	| 'end'
> & { fontId: number; fontMetadata: COMP.FontMetadata };

export interface TToTransformFrameNode extends TToTransformBaseNode {
	type: 'Frame';
	node: FrameNode | ComponentNode | InstanceNode;
	childrenIds: TContinuousId[];
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	isRoot: boolean;
}

export interface TToTransformGroupNode extends TToTransformBaseNode {
	type: 'Group';
	node: GroupNode;
	childrenIds: TContinuousId[];
}

export interface TToTransformShapeNode extends TToTransformBaseNode {
	type: 'Shape';
	node: TFigmaShapeNode;
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}

export interface TToTransformUncategorizedSceneNode extends TToTransformBaseNode {
	type: 'Uncategorized';
	node: SceneNode;
}

export type TToTransformNode =
	| TToTransformTextNode
	| TToTransformFrameNode
	| TToTransformGroupNode
	| TToTransformShapeNode
	| TToTransformUncategorizedSceneNode;

export interface TToTransformPaint {
	id: TContinuousId;
	nodeIds: SceneNode['id'][];
	paint: TToTransformPaintVariant;
}

export type TToTransformPaintVariant =
	| SolidPaint
	| GradientPaint
	| (ImagePaint & { assetId: TContinuousId });

export interface TToTransformAsset {
	id: TContinuousId;
	nodeIds: SceneNode['id'][];
	asset: TToTransformImageAsset | TToTransformFontAsset;
}

export interface TToTransformImageAsset {
	type: 'Image';
	hash: string | null;
}

export interface TToTransformFontAsset {
	type: 'Font';
	metadata: COMP.FontMetadata;
}

export interface TToTransformFill {
	paintId: TContinuousId;
	visible: boolean;
	blendMode: BlendMode;
	opacity: number;
}

export interface TToTransformStroke {
	paintId: TContinuousId;
	visible: boolean;
	blendMode: BlendMode;
	opacity: number;
	width: number;
}

interface TToTransformHashmapItem {
	id: TContinuousId;
	index: number;
}
