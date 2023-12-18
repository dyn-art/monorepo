import type { TComposition, TFontMetadata, TFontWithContent, TNode, TPaint } from '@dyn/dtif';
import { ContinuousId, type TContinuousId } from '@dyn/utils';

import { transformFont, transformNode, transformPaint } from './transform';
import { dropMixed, hasChildrenFigma, hasFillFigma, isFigmaTextNode } from './utils';

export class Transformer {
	// Figma Nodes
	private _toTransformRootNode: FrameNode;
	private _toTransformNodes: TToTransformNode[] = [];
	private _nodesFailedToTransform: TToTransformNode[] = [];

	// DTIF Nodes
	public readonly nodes = new Map<number, TNode>();
	private _rootNodeId: number;

	// Figma Paints
	private _toTransformPaints: TToTransformPaint[] = [];
	private _paintsFailedToTransform: TToTransformPaint[] = [];

	// DTIF Paints
	public readonly paints = new Map<number, TPaint>();

	// Fonts
	private _toTransformFonts: TToTransformFont[] = [];
	private _fontsFailedToTransform: TToTransformFont[] = [];

	// DTIF Fonts
	public readonly fonts = new Map<number, TFontWithContent>();

	constructor(node: FrameNode) {
		this._toTransformRootNode = node;
	}

	public async transform(): Promise<TComposition> {
		const rootId = this.traverseFigmaNodeTree(this._toTransformRootNode);
		this._rootNodeId = rootId.toNumber();

		// Transform nodes
		await this.transformNodes();

		// Transform paints
		await this.transformPaints();

		// Transform fonts
		await this.transformFonts();

		// Construct composition
		const composition: TComposition = {
			version: '1.0',
			name: this._toTransformRootNode.name,
			width: this._toTransformRootNode.width,
			height: this._toTransformRootNode.height,
			nodes: Object.fromEntries(this.nodes),
			paints: Object.fromEntries(this.paints),
			fonts: Object.fromEntries(this.fonts),
			rootNodeId: this._rootNodeId
		};

		return composition;
	}

	// =========================================================================
	// Traverse
	// =========================================================================

	private traverseFigmaNodeTree(root: FrameNode): ContinuousId {
		const rootId = ContinuousId.ZERO;
		const toTransformPaintsMap = new Map<string, TContinuousId>();
		const toTransformFontsMap = new Map<string, TContinuousId>();

		this._toTransformRootNode = root;
		this._toTransformNodes = [];
		this._toTransformPaints = [];

		// Generates a unique ID for an item, if not already generated
		const getOrGenerateId = <T>(
			map: Map<string, TContinuousId>,
			toTransformArray: T[],
			value: T
		): TContinuousId => {
			const key = JSON.stringify(value);
			let id = map.get(key);
			if (!id) {
				id = ContinuousId.nextId();
				toTransformArray.push(value);
				map.set(key, id);
			}
			return id;
		};

		// Walks through each node and processes children, paints, and fonts
		const walk = (node: SceneNode, isRoot = false): TContinuousId => {
			const nodeId = isRoot ? rootId.toNumber() : ContinuousId.nextId();
			const childrenIds = hasChildrenFigma(node)
				? node.children.map((child) => walk(child))
				: undefined;
			const paintIds = processPaints(node, toTransformPaintsMap, this._toTransformPaints);
			const fontIds = processFonts(node, toTransformFontsMap, this._toTransformFonts);

			this._toTransformNodes.push({
				id: nodeId,
				node,
				childrenIds,
				paintIds,
				fontIds
			});

			return nodeId;
		};

		// Processes node paints and returns their IDs
		const processPaints = (
			node: SceneNode,
			map: Map<string, TContinuousId>,
			paintsArray: TToTransformPaint[]
		): TContinuousId[] | undefined => {
			if (!hasFillFigma(node)) {
				return undefined;
			}
			const fills = dropMixed(node, 'fills');
			return fills.map((paint) =>
				getOrGenerateId(map, paintsArray, { id: ContinuousId.nextId(), paint })
			);
		};

		// Processes node fonts and returns their ID
		// TODO: Support multipe text sections
		const processFonts = (
			node: SceneNode,
			map: Map<string, TContinuousId>,
			fontsArray: TToTransformFont[]
		): TContinuousId[] | undefined => {
			if (!isFigmaTextNode(node)) {
				return undefined;
			}
			const { family, style } = dropMixed(node, 'fontName');
			const fontWeight = dropMixed(node, 'fontWeight');
			const fontMetadata = {
				family,
				name: style,
				weight: fontWeight,
				style: style.toLowerCase().includes('italic') ? 'Italic' : 'Normal'
			};
			return [getOrGenerateId(map, fontsArray, { id: ContinuousId.nextId(), fontMetadata })];
		};

		// Start walking the Figma node tree from the root
		walk(root, true);

		return rootId;
	}

	// =========================================================================
	// Transform
	// =========================================================================

	private async transformNodes(): Promise<void> {
		const toTransformNodes = this._toTransformNodes
			.splice(0, this._toTransformNodes.length)
			.concat(this._nodesFailedToTransform.splice(0, this._nodesFailedToTransform.length));

		// Transform nodes
		for (const toTransformNode of toTransformNodes) {
			try {
				const node = await transformNode(toTransformNode);
				this.nodes.set(toTransformNode.id, node);
			} catch (error) {
				// TODO: Error
				this._nodesFailedToTransform.push(toTransformNode);
			}
		}
	}

	private async transformPaints(): Promise<void> {
		const toTransformPaints = this._toTransformPaints
			.splice(0, this._toTransformPaints.length)
			.concat(this._paintsFailedToTransform.splice(0, this._paintsFailedToTransform.length));

		// Transform paints
		for (const toTransformPaint of toTransformPaints) {
			try {
				const paint = await transformPaint(toTransformPaint.paint);
				this.paints.set(toTransformPaint.id, paint);
			} catch (error) {
				// TODO: Error
				this._paintsFailedToTransform.push(toTransformPaint);
			}
		}
	}

	private async transformFonts(): Promise<void> {
		const toTransformFonts = this._toTransformFonts
			.splice(0, this._toTransformFonts.length)
			.concat(this._fontsFailedToTransform.splice(0, this._fontsFailedToTransform.length));

		// Transform fonts
		for (const toTransformFont of toTransformFonts) {
			try {
				const font = await transformFont(toTransformFont.fontMetadata);
				this.fonts.set(toTransformFont.id, font);
			} catch (error) {
				// TODO: Error
				this._fontsFailedToTransform.push(toTransformFont);
			}
		}
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
	paint: Paint;
}

export interface TToTransformFont {
	id: TContinuousId;
	fontMetadata: TFontMetadata;
}
