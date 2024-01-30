import { MD5 } from 'object-hash';
import type { COMP } from '@dyn/dtif';
import { ContinuousId, type TContinuousId } from '@dyn/utils';

import { UnsupportedFigmaNodeException } from './exceptions';
import type { TFigmaNodeWithChildren, TFigmaNodeWithPaints, TFigmaShapeNode } from './types';
import {
	dropMixed,
	isFigmaComponentNode,
	isFigmaFrameNode,
	isFigmaGroupNode,
	isFigmaInstanceNode,
	isFigmaShapeNode,
	isFigmaTextNode
} from './utils';

export class FigmaNodeTreeProcessor {
	private _root: FrameNode;
	private _toTransformNodes: TToTransformNode[] = [];

	private _toTransformPaints: TToTransformPaint[] = [];
	private _toTransformPaintsHashmap = new Map<string, TToTransformHashmapItem>();

	private _toTransformFonts: TToTransformFont[] = [];
	private _toTransformFontsHashmap = new Map<string, TToTransformHashmapItem>();

	constructor(root: FrameNode) {
		this._root = root;
	}

	// Entry method to start processing the node tree
	public processNodeTree(): {
		rootId: TContinuousId;
		toTransformNodes: TToTransformNode[];
		toTransformPaints: TToTransformPaint[];
		toTransformFonts: TToTransformFont[];
	} {
		const rootId = this.walk(this._root, true);

		return {
			rootId,
			toTransformNodes: this._toTransformNodes,
			toTransformPaints: this._toTransformPaints,
			toTransformFonts: this._toTransformFonts
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
				paintIds: this.processPaints(node),
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
				segments: this.processFonts(node),
				paintIds: this.processPaints(node)
			});
		} else if (isFigmaShapeNode(node)) {
			this._toTransformNodes.push({
				type: 'Shape',
				id: nodeId,
				node,
				paintIds: this.processPaints(node)
			});
		} else {
			throw new UnsupportedFigmaNodeException(node);
		}

		return nodeId;
	}

	// Processes children of a node
	private processChildren(node: TFigmaNodeWithChildren): TContinuousId[] {
		return node.children.map((child) => this.walk(child));
	}

	// Processes paints of a node
	private processPaints(node: TFigmaNodeWithPaints): TContinuousId[] {
		const fills = dropMixed(node, 'fills');
		return fills.map((paint) =>
			this.getOrGenerateId(this._toTransformPaintsHashmap, this._toTransformPaints, {
				nodeIds: [node.id],
				paint
			})
		);
	}

	// Processes fonts of a node
	private processFonts(node: TextNode): TTextNodeSegment[] {
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
			fontId: this.getOrGenerateId(this._toTransformFontsHashmap, this._toTransformFonts, {
				nodeIds: [node.id],
				fontMetadata: this.extractFontMetadata(segment)
			})
		}));
	}

	// Helper to extract font metadata from a node
	private extractFontMetadata(segment: Omit<TTextNodeSegment, 'fontId'>): COMP.FontMetadata {
		return {
			family: segment.fontName.family,
			name: segment.fontName.style,
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
	type: 'Text' | 'Frame' | 'Group' | 'Shape';
	id: TContinuousId;
	node: SceneNode;
}

export interface TToTransformTextNode extends TToTransformBaseNode {
	type: 'Text';
	node: TextNode;
	segments: TTextNodeSegment[];
	paintIds: TContinuousId[];
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
> & { fontId: number };

export interface TToTransformFrameNode extends TToTransformBaseNode {
	type: 'Frame';
	node: FrameNode | ComponentNode | InstanceNode;
	childrenIds: TContinuousId[];
	paintIds: TContinuousId[];
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
	paintIds: TContinuousId[];
}

export type TToTransformNode =
	| TToTransformTextNode
	| TToTransformFrameNode
	| TToTransformGroupNode
	| TToTransformShapeNode;

export interface TToTransformPaint {
	id: TContinuousId;
	nodeIds: SceneNode['id'][];
	paint: Paint;
}

export interface TToTransformFont {
	id: TContinuousId;
	nodeIds: SceneNode['id'][];
	fontMetadata: COMP.FontMetadata;
}

interface TToTransformHashmapItem {
	id: TContinuousId;
	index: number;
}
