import type { TComposition, TFont, TNode, TPaint } from '@dyn/dtif';

import { FailedToResolveRootNodeException } from './exceptions';
import {
	FigmaNodeTreeProcessor,
	type TToTransformFont,
	type TToTransformNode,
	type TToTransformPaint
} from './FigmaNodeTreeProcessor';
import {
	transformFont,
	transformNode,
	transformPaint,
	type TTransformFontConfig,
	type TTransformNodeConfig,
	type TTransformPaintConfig
} from './transform';
import { resetDTIFNodeTransform } from './utils';

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
	public readonly fonts = new Map<number, TFont>();

	// Callbacks
	private _onTransformStatusUpdate: TOnTransformStatusUpdate | null = null;

	constructor(node: FrameNode, options: TTransformerOptions = {}) {
		const { onTransformStatusUpdate = null } = options;
		this._toTransformRootNode = node;
		this._onTransformStatusUpdate = onTransformStatusUpdate;
	}

	public async transform(config: TTransformConfig): Promise<TComposition> {
		const nodeConfig: TTransformNodeConfig = { includeInvisible: true, ...(config.node ?? {}) };
		const paintConfig = config.paint;
		const fontConfig = config.font;

		this._onTransformStatusUpdate?.({ type: ETransformStatus.START });

		// Walk Figma tree and discover to transform nodes, paints and fonts
		const { rootId, toTransformNodes, toTransformPaints, toTransformFonts } =
			new FigmaNodeTreeProcessor(this._toTransformRootNode).processNodeTree();
		this._rootNodeId = rootId;
		this._toTransformNodes = toTransformNodes;
		this._toTransformPaints = toTransformPaints;
		this._toTransformFonts = toTransformFonts;

		this._onTransformStatusUpdate?.({
			type: ETransformStatus.TRAVERSED_TREE,
			toTransformNodesCount: this._toTransformNodes.length,
			toTransformPaintsCount: this._toTransformPaints.length,
			toTransformFontsCount: this._toTransformFonts.length
		});

		// Transform nodes
		this._onTransformStatusUpdate?.({ type: ETransformStatus.TRANSFORMING_NODES });
		await this.transformNodes(nodeConfig);

		// Transform paints
		this._onTransformStatusUpdate?.({ type: ETransformStatus.TRANSFORMING_PAINTS });
		await this.transformPaints(paintConfig);

		// Transform fonts
		this._onTransformStatusUpdate?.({ type: ETransformStatus.TRANSFORMING_FONTS });
		await this.transformFonts(fontConfig);

		// Reset root node layout
		const rootNode = this.nodes.get(this._rootNodeId);
		if (rootNode != null) {
			resetDTIFNodeTransform(rootNode);
		} else {
			throw new FailedToResolveRootNodeException();
		}

		// Construct composition
		this._onTransformStatusUpdate?.({ type: ETransformStatus.CONSTRUCTING_COMPOSITON });
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

		this._onTransformStatusUpdate?.({ type: ETransformStatus.END });
		return composition;
	}

	// =========================================================================
	// Transform
	// =========================================================================

	private async transformNodes(config: TTransformNodeConfig): Promise<void> {
		const toTransformNodes = this._toTransformNodes
			.splice(0, this._toTransformNodes.length)
			.concat(this._nodesFailedToTransform.splice(0, this._nodesFailedToTransform.length));

		// Transform nodes
		for (const toTransformNode of toTransformNodes) {
			try {
				const node = await transformNode(toTransformNode, config);
				this.nodes.set(toTransformNode.id, node);
			} catch (error) {
				// TODO: Error
				this._nodesFailedToTransform.push(toTransformNode);
			}
		}
	}

	private async transformPaints(config: TTransformPaintConfig): Promise<void> {
		const toTransformPaints = this._toTransformPaints
			.splice(0, this._toTransformPaints.length)
			.concat(this._paintsFailedToTransform.splice(0, this._paintsFailedToTransform.length));

		// Transform paints
		for (const toTransformPaint of toTransformPaints) {
			try {
				const paint = await transformPaint(toTransformPaint, config);
				this.paints.set(toTransformPaint.id, paint);
			} catch (error) {
				// TODO: Error
				this._paintsFailedToTransform.push(toTransformPaint);
			}
		}
	}

	private async transformFonts(config: TTransformFontConfig): Promise<void> {
		const toTransformFonts = this._toTransformFonts
			.splice(0, this._toTransformFonts.length)
			.concat(this._fontsFailedToTransform.splice(0, this._fontsFailedToTransform.length));

		// Transform fonts
		for (const toTransformFont of toTransformFonts) {
			try {
				const font = await transformFont(toTransformFont, config);
				this.fonts.set(toTransformFont.id, font);
			} catch (error) {
				// TODO: Error
				this._fontsFailedToTransform.push(toTransformFont);
			}
		}
	}
}

export interface TTransformConfig {
	node?: Partial<TTransformNodeConfig>;
	font: TTransformFontConfig;
	paint: TTransformPaintConfig;
}

export interface TTransformerOptions {
	onTransformStatusUpdate?: TOnTransformStatusUpdate;
}

export type TOnTransformStatusUpdate = (status: TTransformStatusUpdate) => void;

export type TTransformStatusUpdate =
	| { type: ETransformStatus.START }
	| {
			type: ETransformStatus.TRAVERSED_TREE;
			toTransformNodesCount: number;
			toTransformPaintsCount: number;
			toTransformFontsCount: number;
	  }
	| { type: ETransformStatus.TRANSFORMING_NODES }
	| { type: ETransformStatus.TRANSFORMING_PAINTS }
	| { type: ETransformStatus.TRANSFORMING_FONTS }
	| { type: ETransformStatus.CONSTRUCTING_COMPOSITON }
	| { type: ETransformStatus.END };

export enum ETransformStatus {
	START,
	TRAVERSED_TREE,
	TRANSFORMING_NODES,
	TRANSFORMING_PAINTS,
	TRANSFORMING_FONTS,
	CONSTRUCTING_COMPOSITON,
	END
}

export type TOnTraversedTree = (data: {
	toTransformNodesCount: number;
	toTransformPaintsCount: number;
	toTransformFontsCount: number;
}) => void;
