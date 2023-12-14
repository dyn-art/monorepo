import type {
	DTIFComposition,
	FontMetadata as DTIFFontMetadata,
	FontWithContent as DTIFFontWithContent,
	Node as DTIFNode,
	Paint as DTIFPaint
} from '@dyn/svg-composition/bindings';

import { dropMixed, hasChildren, hasFill, isTextNode } from './utils/figma-node';
import { ContinuousId } from './utils/math';

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

	constructor(node: FrameNode) {
		this._toTransformRootNode = node;
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

	public traverseFigmaNodeTree(root: FrameNode): ContinuousId {
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
			const childrenIds = hasChildren(node) ? node.children.map((child) => walk(child)) : [];
			const paintIds = processPaints(node, toTransformPaintsMap, this._toTransformPaints);
			const fontId = processFonts(node, toTransformFontsMap, this._toTransformFonts);

			this._toTransformNodes.push({
				id: nodeId,
				node,
				childrenIds,
				paintIds,
				fontId: fontId ?? undefined
			});

			return nodeId;
		};

		// Processes node paints and returns their IDs
		const processPaints = (
			node: SceneNode,
			map: Map<string, ContinuousId>,
			paintsArray: any[]
		): ContinuousId[] => {
			if (!hasFill(node)) return [];
			const fills = dropMixed(node, 'fills').unwrap();
			return fills.map((paint) => getOrGenerateId(map, paintsArray, { paint }));
		};

		// Processes node fonts and returns their ID
		const processFonts = (
			node: SceneNode,
			map: Map<string, ContinuousId>,
			fontsArray: any[]
		): ContinuousId | null => {
			if (!isTextNode(node)) return null;
			const { family, style } = dropMixed(node, 'fontName').unwrap();
			const fontWeight = dropMixed(node, 'fontWeight').unwrap();
			const fontMetadata = {
				family,
				name: style,
				weight: fontWeight,
				style: style.toLowerCase().includes('italic') ? 'Italic' : 'Normal'
			};
			return getOrGenerateId(map, fontsArray, { fontMetadata });
		};

		// Start walking the Figma node tree from the root
		walk(root, true);

		return rootId;
	}
}

interface TToTransformNode {
	id: ContinuousId;
	node: SceneNode;
	childrenIds: ContinuousId[];
	paintIds: ContinuousId[];
	fontId?: ContinuousId;
}

interface TToTransformPaint {
	id: ContinuousId;
	paint: Paint;
}

interface TToTransformFont {
	id: ContinuousId;
	fontMetadata: DTIFFontMetadata;
}
