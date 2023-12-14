import type {
	DTIFComposition,
	FontMetadata as DTIFFontMetadata,
	FontWithContent as DTIFFontWithContent,
	Node as DTIFNode,
	Paint as DTIFPaint
} from '@dyn/svg-composition/bindings';

import { transformNode } from './transform';
import {
	ContinuousId,
	dropMixed,
	hasChildrenDTIF,
	hasChildrenFigma,
	hasFillDTIF,
	hasFillFigma,
	isDTIFTextNode,
	isFigmaTextNode
} from './utils';

export class Transformer {
	// Figma Nodes
	private _toTransformRootNode: FrameNode;
	private _toTransformNodes: TToTransformNode[] = [];
	private _nodesFailedToTransform: TToTransformNode[] = [];

	// DTIF Nodes
	public readonly nodes = new Map<number, DTIFNode>();
	private _rootNodeId: number;

	// Figma Paints
	private _toTransformPaints: TToTransformPaint[] = [];
	private _paintsFailedToTransform: TToTransformPaint[] = [];

	// DTIF Paints
	public readonly paints = new Map<number, DTIFPaint>();

	// Fonts
	private _toTransformFonts: TToTransformFont[] = [];
	private _fontsFailedToTransform: TToTransformFont[] = [];

	// DTIF Fonts
	public readonly fonts = new Map<number, DTIFFontWithContent>();

	public static readonly supportedNodeTypes = [
		'FRAME',
		'COMPONENT',
		'INSTANCE',
		'GROUP',
		'TEXT',
		'RECTANGLE',
		'LINE',
		'ELLIPSE',
		'POLYGON',
		'STAR',
		'VECTOR',
		'BOOLEAN_OPERATION'
	];

	constructor(node: FrameNode) {
		this._toTransformRootNode = node;
	}

	public static isSupportedNodeType(type: string): boolean {
		return this.supportedNodeTypes.includes(type);
	}

	public async transform(): Promise<DTIFComposition> {
		const rootId = this.traverseFigmaNodeTree(this._toTransformRootNode);
		this._rootNodeId = rootId.toNumber();

		// Transform nodes
		// TODO

		// Transform paints
		// TODO

		// Transform fonts
		// TODO

		// Construct composition
		// TODO

		return null as any;
	}

	// =========================================================================
	// Traverse
	// =========================================================================

	private traverseFigmaNodeTree(root: FrameNode): ContinuousId {
		const rootId = ContinuousId.ZERO;
		const toTransformPaintsMap = new Map<string, ContinuousId>();
		const toTransformFontsMap = new Map<string, ContinuousId>();

		this._toTransformRootNode = root;
		this._toTransformNodes = [];
		this._toTransformPaints = [];

		// Generates a unique ID for an item, if not already generated
		const getOrGenerateId = (
			map: Map<string, ContinuousId>,
			toTransformArray: any[],
			value: any
		): ContinuousId => {
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
		const walk = (node: SceneNode, isRoot = false): ContinuousId => {
			const nodeId = isRoot ? rootId : ContinuousId.nextId();
			const childrenIds = hasChildrenFigma(node) ? node.children.map((child) => walk(child)) : [];
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
			map: Map<string, ContinuousId>,
			paintsArray: any[]
		): ContinuousId[] => {
			if (!hasFillFigma(node)) {
				return [];
			}
			const fills = dropMixed(node, 'fills');
			return fills.map((paint) => getOrGenerateId(map, paintsArray, { paint }));
		};

		// Processes node fonts and returns their ID
		// TODO: Support multipe text sections
		const processFonts = (
			node: SceneNode,
			map: Map<string, ContinuousId>,
			fontsArray: any[]
		): ContinuousId[] => {
			if (!isFigmaTextNode(node)) {
				return [];
			}
			const { family, style } = dropMixed(node, 'fontName');
			const fontWeight = dropMixed(node, 'fontWeight');
			const fontMetadata = {
				family,
				name: style,
				weight: fontWeight,
				style: style.toLowerCase().includes('italic') ? 'Italic' : 'Normal'
			};
			return [getOrGenerateId(map, fontsArray, { fontMetadata })];
		};

		// Start walking the Figma node tree from the root
		walk(root, true);

		return rootId;
	}

	// =========================================================================
	// Transform
	// =========================================================================

	private async transformNodes() {
		const toTransformNodes = this._toTransformNodes.splice(0, this._toTransformNodes.length);

		// Transform nodes
		for (const toTransformNode of toTransformNodes) {
			const node = await transformNode(toTransformNode.node);
			if (hasChildrenDTIF(node)) {
				node.children = toTransformNode.childrenIds.map((id) => id.toNumber());
			}
			if (hasFillDTIF(node)) {
				node.fill = { paintIds: toTransformNode.paintIds.map((id) => id.toNumber()) };
			}
			if (isDTIFTextNode(node)) {
				node.text.sections.forEach((section, index) => {
					const fontId = toTransformNode.fontIds[index];
					if (fontId != null) {
						section.style.fontId = fontId.toNumber();
					} else {
						// TODO: Error
					}
				});
			}
			this.nodes.set(toTransformNode.id.toNumber(), node);
		}
	}

	private async transformPaints() {
		// TODO
	}

	private async transformFonts() {
		// TODO
	}
}

interface TToTransformNode {
	id: ContinuousId;
	node: SceneNode;
	childrenIds: ContinuousId[];
	paintIds: ContinuousId[];
	fontIds: ContinuousId[];
}

interface TToTransformPaint {
	id: ContinuousId;
	paint: Paint;
}

interface TToTransformFont {
	id: ContinuousId;
	fontMetadata: DTIFFontMetadata;
}
