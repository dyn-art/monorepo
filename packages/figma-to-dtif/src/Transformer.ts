import { ContinuousId, sleep } from '@ibg/utils';
import type { COMP } from '@dyn/comp-dtif';

import { FailedToResolveRootNodeException } from './exceptions';
import {
	FigmaNodeTreeProcessor,
	type TToTransformAsset,
	type TToTransformNode,
	type TToTransformPaint
} from './FigmaNodeTreeProcessor';
import {
	transformAsset,
	transformNode,
	transformPaint,
	type TTransformAssetConfig,
	type TTransformNodeConfig
} from './transform';
import { resetDtifNodeTransform } from './utils';

export class Transformer {
	// Figma Nodes
	private _toTransformRootNode: FrameNode;
	private _toTransformNodes: TToTransformNode[] = [];
	private _nodesFailedToTransform: TToTransformNode[] = [];

	// DTIF Nodes
	public readonly nodes: COMP.Node[] = [];
	private _rootNodeId: string;

	// Figma Paints
	private _toTransformPaints: TToTransformPaint[] = [];
	private _paintsFailedToTransform: TToTransformPaint[] = [];

	// DTIF Paints
	public readonly paints: COMP.Paint[] = [];

	// Assets
	private _toTransformAssets: TToTransformAsset[] = [];
	private _assetsFailedToTransform: TToTransformAsset[] = [];

	// DTIF Assets
	public readonly assets: COMP.AssetWithId[] = [];

	// Callbacks
	private _onTransformStatusUpdate: TOnTransformStatusUpdate | null = null;

	constructor(node: FrameNode, options: TTransformerOptions = {}) {
		const { onTransformStatusUpdate = null } = options;
		this._toTransformRootNode = node;
		this._onTransformStatusUpdate = onTransformStatusUpdate;
	}

	public async transform(config: TTransformConfig): Promise<COMP.DtifComposition> {
		ContinuousId.ZERO;
		const nodeConfig: TTransformNodeConfig = {
			includeInvisible: true,
			exportContainerNode: this.createExportContainerNode(
				'⏳ Temp export container | Delete if dyn.art plugin not active'
			),
			shouldExportFrame: { format: 'SVG' },
			...(config.node ?? {})
		};

		await this.onTransformStatusUpdate({ type: ETransformStatus.START });

		// Walk Figma tree and discover to transform nodes, paints and assets
		const { rootId, toTransformNodes, toTransformPaints, toTransformAssets } =
			new FigmaNodeTreeProcessor(this._toTransformRootNode).processNodeTree();
		this._rootNodeId = `n${rootId}`;
		this._toTransformNodes = toTransformNodes;
		this._toTransformPaints = toTransformPaints;
		this._toTransformAssets = toTransformAssets;

		this._onTransformStatusUpdate?.({
			type: ETransformStatus.TRAVERSED_TREE,
			toTransformNodesCount: this._toTransformNodes.length,
			toTransformPaintsCount: this._toTransformPaints.length,
			toTransformAssetsCount: this._toTransformAssets.length
		});

		// Transform nodes
		await this.onTransformStatusUpdate({ type: ETransformStatus.TRANSFORMING_NODES });
		await this.transformNodes(nodeConfig);

		// Transform paints
		await this.onTransformStatusUpdate({ type: ETransformStatus.TRANSFORMING_PAINTS });
		await this.transformPaints();

		// Transform assets
		await this.onTransformStatusUpdate({ type: ETransformStatus.TRANSFORMING_ASSETS });
		await this.transformAssets(config.asset);

		// Reset root node layout
		const rootNode = this.nodes.find((node) => node.id === this._rootNodeId);
		if (rootNode != null) {
			resetDtifNodeTransform(rootNode);
		} else {
			throw new FailedToResolveRootNodeException();
		}

		// Construct composition
		await this.onTransformStatusUpdate({ type: ETransformStatus.CONSTRUCTING_COMPOSITON });
		const composition: COMP.DtifComposition = {
			version: 'V000001',
			size: [this._toTransformRootNode.width, this._toTransformRootNode.height],
			nodes: this.nodes,
			paints: this.paints,
			assets: this.assets,
			viewport: {
				physicalPosition: [0, 0],
				physicalSize: [this._toTransformRootNode.width, this._toTransformRootNode.height]
			}
		};

		nodeConfig.exportContainerNode.remove();
		await this.onTransformStatusUpdate({ type: ETransformStatus.END });
		return composition;
	}

	private async onTransformStatusUpdate(status: TTransformStatusUpdate): Promise<void> {
		this._onTransformStatusUpdate?.(status);
		// Sleep to give Figma time to send the status update to the "frontend"
		// before it might get blocked e.g. due to image export
		await sleep(50);
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
				const node = await transformNode(toTransformNode, this, config);
				this.insertNode(toTransformNode.id, node);
			} catch (error) {
				// TODO: Error
				this._nodesFailedToTransform.push(toTransformNode);
			}
		}
	}

	public insertNode(id: number, node: COMP.Node): void {
		node.id = `n${id}`;
		this.nodes.push(node);
	}

	private async transformPaints(): Promise<void> {
		const toTransformPaints = this._toTransformPaints
			.splice(0, this._toTransformPaints.length)
			.concat(this._paintsFailedToTransform.splice(0, this._paintsFailedToTransform.length));

		// Transform paints
		for (const toTransformPaint of toTransformPaints) {
			try {
				const paint = await transformPaint(toTransformPaint);
				this.insertPaint(toTransformPaint.id, paint);
			} catch (error) {
				// TODO: Error
				this._paintsFailedToTransform.push(toTransformPaint);
			}
		}
	}

	public insertPaint(id: number, paint: COMP.Paint): void {
		paint.id = `p${id}`;
		this.paints.push(paint);
	}

	private async transformAssets(config: TTransformAssetConfig): Promise<void> {
		const toTransformAssets = this._toTransformAssets
			.splice(0, this._toTransformAssets.length)
			.concat(this._assetsFailedToTransform.splice(0, this._assetsFailedToTransform.length));

		// Transform assets
		for (const toTransformAsset of toTransformAssets) {
			try {
				const asset = await transformAsset(toTransformAsset, config);
				this.insertAsset(toTransformAsset.id, asset);
			} catch (error) {
				// TODO: Error
				this._assetsFailedToTransform.push(toTransformAsset);
			}
		}
	}

	public insertAsset(id: number, asset: COMP.AssetWithId): void {
		asset.id = `a${id}`;
		this.assets.push(asset);
	}

	private createExportContainerNode(name: string): FrameNode {
		const node = figma.createFrame();
		node.name = name;
		node.resize(1, 1);
		node.clipsContent = false; // With clip content active figma would just export the visible piece in the frame
		return node;
	}
}

export interface TTransformConfig {
	node?: Partial<TTransformNodeConfig>;
	asset: TTransformAssetConfig;
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
			toTransformAssetsCount: number;
	  }
	| { type: ETransformStatus.TRANSFORMING_NODES }
	| { type: ETransformStatus.TRANSFORMING_PAINTS }
	| { type: ETransformStatus.TRANSFORMING_ASSETS }
	| { type: ETransformStatus.CONSTRUCTING_COMPOSITON }
	| { type: ETransformStatus.END };

export enum ETransformStatus {
	START,
	TRAVERSED_TREE,
	TRANSFORMING_NODES,
	TRANSFORMING_PAINTS,
	TRANSFORMING_ASSETS,
	CONSTRUCTING_COMPOSITON,
	END
}
