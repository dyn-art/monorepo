import { MD5 } from 'object-hash';
import type { TFontMetadata } from '@dyn/dtif';
import { ContinuousId, type TContinuousId } from '@dyn/utils';

import { dropMixed, hasChildrenFigma, hasFillFigma, isFigmaTextNode } from './utils';

export class FigmaNodeTreeProcessor {
	private root: FrameNode;
	private toTransformNodes: TToTransformNode[] = [];

	private toTransformPaints: TToTransformPaint[] = [];
	private toTransformPaintsMap = new Map<string, TToTransformItem>();

	private toTransformFonts: TToTransformFont[] = [];
	private toTransformFontsMap = new Map<string, TToTransformItem>();

	constructor(root: FrameNode) {
		this.root = root;
	}

	// Entry method to start processing the node tree
	public processNodeTree(): {
		rootId: TContinuousId;
		toTransformNodes: TToTransformNode[];
		toTransformPaints: TToTransformPaint[];
		toTransformFonts: TToTransformFont[];
	} {
		const rootId = ContinuousId.ZERO;
		this.walk(this.root, true);
		return {
			rootId: rootId.toNumber(),
			toTransformNodes: this.toTransformNodes,
			toTransformPaints: this.toTransformPaints,
			toTransformFonts: this.toTransformFonts
		};
	}

	// Recursive method to walk through each node
	private walk(node: SceneNode, isRoot = false): TContinuousId {
		const nodeId = isRoot ? ContinuousId.ZERO.toNumber() : ContinuousId.nextId();
		const childrenIds = this.processChildren(node);
		const paintIds = this.processPaints(node);
		const fontIds = this.processFonts(node);

		this.toTransformNodes.push({ id: nodeId, node, childrenIds, paintIds, fontIds });

		return nodeId;
	}

	// Processes children of a node
	private processChildren(node: SceneNode): TContinuousId[] | undefined {
		return hasChildrenFigma(node) ? node.children.map((child) => this.walk(child)) : undefined;
	}

	// Processes paints of a node
	private processPaints(node: SceneNode): TContinuousId[] | undefined {
		if (!hasFillFigma(node)) {
			return undefined;
		}

		const fills = dropMixed(node, 'fills');
		return fills.map((paint) =>
			this.getOrGenerateId(this.toTransformPaintsMap, this.toTransformPaints, {
				nodeIds: [node.id],
				paint
			})
		);
	}

	// Processes fonts of a node
	private processFonts(node: SceneNode): TContinuousId[] | undefined {
		if (!isFigmaTextNode(node)) {
			return undefined;
		}

		const fontMetadata = this.extractFontMetadata(node);
		return [
			this.getOrGenerateId(this.toTransformFontsMap, this.toTransformFonts, {
				nodeIds: [node.id],
				fontMetadata
			})
		];
	}

	// Helper to extract font metadata from a node
	private extractFontMetadata(node: TextNode): TFontMetadata {
		const { family, style } = dropMixed(node, 'fontName');
		const fontWeight = dropMixed(node, 'fontWeight');

		return {
			family,
			name: style,
			weight: fontWeight,
			style: style.toLowerCase().includes('italic') ? 'Italic' : 'Normal'
		};
	}

	// Generates a unique ID for an item or retrieves an existing one
	private getOrGenerateId<GValue extends { id: TContinuousId; nodeIds: SceneNode['id'][] }>(
		hashMap: Map<string, TToTransformItem>,
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

export interface TToTransformNode {
	id: TContinuousId;
	node: SceneNode;
	childrenIds?: TContinuousId[];
	paintIds?: TContinuousId[];
	fontIds?: TContinuousId[];
}

export interface TToTransformPaint {
	id: TContinuousId;
	nodeIds: SceneNode['id'][];
	paint: Paint;
}

export interface TToTransformFont {
	id: TContinuousId;
	nodeIds: SceneNode['id'][];
	fontMetadata: TFontMetadata;
}

interface TToTransformItem {
	id: TContinuousId;
	index: number;
}
