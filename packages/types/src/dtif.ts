// Note: The node types are inspired by Figma's node types.
// https://www.figma.com/plugin-docs/api/nodes

export type TSupportedFigmaNodeTypes =
	| 'FRAME'
	| 'COMPONENT'
	| 'INSTANCE'
	| 'GROUP'
	| 'TEXT'
	| 'RECTANGLE'
	| 'LINE'
	| 'ELLIPSE'
	| 'POLYGON'
	| 'STAR'
	| 'VECTOR'
	| 'BOOLEAN_OPERATION';

export type TNodeTypes =
	| 'FRAME'
	| 'GROUP'
	| 'TEXT'
	| 'RECTANGLE'
	| 'EMBED'
	| 'POLYGON'
	| 'STAR'
	| 'ELLIPSE'
	| 'SVG';

// =============================================================================
// Scene
// =============================================================================

/**
 * Represents the composition in which all nodes exist.
 */
export interface TComposition {
	/**
	 * Version of the composition type declaration (for internal use).
	 */
	version: '1.0';
	/**
	 * The name of the composition.
	 *
	 * e.g. 'My super cool composition'
	 */
	name: string;
	/**
	 * The width of the composition.
	 */
	width: number;
	/**
	 * The height of the composition.
	 */
	height: number;
	/**
	 * The root node id of the composition.
	 */
	rootId: string;
	/**
	 * Nodes of the composition.
	 */
	nodes: Record<string, TNode>;
	/**
	 * Paints of the composition.
	 */
	paints: Record<string, TPaint>;
	/**
	 * Typefaces of the composition.
	 */
	typefaces: Record<string, TTypeface>;
}

// =============================================================================
// Nodes
// =============================================================================

/**
 * The frame node is a container used to define a layout hierarchy.
 * It is similar to <div> in HTML.
 * It is different from GroupNode, which is closer to a folder for layers.
 */
export type TFrameNode = {
	type: 'FRAME';
	/**
	 * A boolean indicating whether the frame clips its content to its bounding box.
	 */
	clipsContent: boolean;
} & TShapeNodeMixin &
	TChildrenMixin &
	TRectangleCornerMixin &
	TConstraintsMixin;

/**
 * The group node is a container used to semantically group related nodes. You can think of them as a folder in the layers panel.
 * It is different from FrameNode,
 * which defines layout and is closer to a <div> in HTML.
 *
 * Groups are always positioned and sized to fit their content.
 * As such, while you can move or resize a group,
 * you should also expect that a group's position and size will change
 * if you change its content.
 */
export type TGroupNode = {
	type: 'GROUP';
} & TCompositionNodeMixin &
	TChildrenMixin;

/**
 * The rectangle node is a basic shape node representing a rectangle.
 */
export type TRectangleNode = {
	type: 'RECTANGLE';
} & TShapeNodeMixin &
	TRectangleCornerMixin;

/**
 * The ellipse node is a basic shape node representing an ellipse.
 * Note that a circle is an ellipse where width == height.
 */
export type TEllipseNode = {
	type: 'ELLIPSE';
	/**
	 * Exposes the values of the sweep
	 * and ratio handles used in our UI to create arcs and donuts.
	 */
	arcData: TEllipseArcData;
} & TShapeNodeMixin;

/**
 * The star node is a basic shape node representing
 * a star with a set number of points.
 */
export type TStarNode = {
	type: 'STAR';
	/**
	 * Number of "spikes", or outer points of the star. Must be an integer \>= 3.
	 */
	pointCount: number;
	/**
	 * Ratio of the inner radius to the outer radius.
	 */
	innerRadiusRation: number;
} & TShapeNodeMixin;

/**
 * The polygon node is a basic shape node representing
 * a regular convex polygon with three or more sides.
 */
export type TPolygonNode = {
	type: 'POLYGON';
	/**
	 * Number of sides of the polygon. Must be an integer \>= 3.
	 */
	pointCount: number;
} & TShapeNodeMixin;

/**
 * The text node represents text where both the whole node
 * or individual character ranges can have properties
 * such as color (paints), font size, font name, etc.
 */
export type TTextNode = {
	type: 'TEXT';
	/**
	 * The horizontal alignment of the text with respect to the textbox.
	 */
	textAlignHorizontal: 'LEFT' | 'CENTER' | 'RIGHT' | 'JUSTIFIED';
	/**
	 * The vertical alignment of the text with respect to the textbox.
	 */
	textAlignVertical: 'TOP' | 'CENTER' | 'BOTTOM';
	/**
	 * The size of the font. Has minimum value of 1.
	 */
	fontSize: number;
	/**
	 * The primary typeface of the text.
	 */
	typefaceId?: string;
	/**
	 * List of fallback typefaces to use when a character doesn't exist in the primary font.
	 */
	fallbackTypefaceIds: string[];
	/**
	 * The spacing between the individual characters.
	 */
	letterSpacing: TLetterSpacing;
	/**
	 * The spacing between the lines in a paragraph of text.
	 */
	lineHeight: TLineHeight;
	/**
	 * The raw characters in the text node.
	 */
	characters: string;
} & TShapeNodeMixin;

/**
 * The SVG node is the most general representation of shape,
 * allowing you to specify individual vertices, segments, and regions.
 *
 * As the name suggest it contains SVG data following the SVG 1.1 standard
 * in either exported or inline form.
 */
export type TSVGNode = TSVGNodeExported | TSVGNodeInline;

/**
 * The SVG node exported represents the SVG node
 * where the SVG content has been exported to an external file.
 */
export type TSVGNodeExported = {
	type: 'SVG';
	/**
	 * A boolean flag indicating that the SVG node's content is exported to an external file.
	 */
	isExported: true;
	/**
	 * The format of the exported file, which can be 'JPG', 'PNG', or 'SVG'.
	 */
	format: 'JPG' | 'PNG' | 'SVG';
	/**
	 * The hash of the exported SVG file. Used to identify the file.
	 */
	hash: string;
	/**
	 * Optional content of the exported SVG file.
	 * It can be either an array of bytes that contains the exported file's data inline,
	 * or a URL string pointing to the file location.
	 *
	 * If not set the content can to be searched by the hash.
	 */
	content?: Uint8Array | string;
} & TCompositionNodeMixin;

/**
 * The SVG node exported represents the SVG node
 * where the SVG content is inline in form of an array of SVG element children.
 */
export type TSVGNodeInline = {
	type: 'SVG';
	/**
	 * A boolean flag indicating that the SVG node's content is inline in the node.
	 */
	isExported: false;
	/**
	 * An array of SVG element children that define the SVG content.
	 */
	children: TSVGElement['children'];
} & TCompositionNodeMixin;

export type TNode =
	| TFrameNode
	| TGroupNode
	| TRectangleNode
	| TEllipseNode
	| TStarNode
	| TPolygonNode
	| TTextNode
	| TSVGNode;

// =============================================================================
// Mixins
// =============================================================================

export type TCompositionNodeMixin = TBaseNodeMixin & TCompositionMixin & TLayoutMixin & TBlendMixin;

export type TShapeNodeMixin = TCompositionNodeMixin &
	TEffectsMixin &
	TGeometryMixin &
	TFillMixin &
	TConstraintsMixin;

export interface TRectangleCornerMixin {
	/**
	 * The number of pixels to round the top left corner of the node.
	 */
	topLeftRadius: number;
	/**
	 * The number of pixels to round the top right corner of the node.
	 */
	topRightRadius: number;
	/**
	 * The number of pixels to round the bottom left corner of the node.
	 */
	bottomLeftRadius: number;
	/**
	 * The number of pixels to round the bottom right corner of the node.
	 */
	bottomRightRadius: number;
}

export interface TBaseNodeMixin {
	/**
	 * The name of the node.
	 *
	 * e.g. 'Cool Node'
	 */
	name: string;
}

export interface TChildrenMixin {
	/**
	 * The list of children node ids, sorted back-to-front.
	 * That is, the first child in the array is the bottommost layer in the scene,
	 * and the last child in the array is the topmost layer.
	 */
	childIds: string[];
}

export interface TLayoutMixin {
	/**
	 * The width of the node.
	 */
	width: number;
	/**
	 * The height of the node.
	 */
	height: number;
	/**
	 * The position of the node relative to its containing parent as a Transform matrix.
	 * Not used for scaling, see width and height instead.
	 */
	relativeTransform: TTransform;
}

export interface TConstraintsMixin {
	/**
	 * Constraints of the node relative to its containing parent.
	 */
	constraints?: {
		horizontal: TConstraintType;
		vertical: TConstraintType;
	};
}

export interface TFillMixin {
	fill: {
		/**
		 * The paintIds used to fill the area of the shape.
		 */
		paintIds: string[];
	};
}

export interface TCompositionMixin {
	/**
	 * Whether the node is visible or not.
	 */
	isVisible: boolean;
	/**
	 * Whether the node is locked or not,
	 * preventing certain user interactions on the canvas such as selecting and dragging.
	 */
	isLocked: boolean;
}

export interface TBlendMixin {
	/**
	 * Blend mode describes how a color blends with what's underneath it.
	 */
	blendMode: TBlendMode;
	/**
	 * The opacity of the node. Must be a value between 0 and 1. Defaults to 1.
	 */
	opacity: number;
	/**
	 * A boolean indicating if the node is used as a mask.
	 * If true, the node masks the content underneath it.
	 */
	isMask: boolean;
}

export interface TEffectsMixin {
	/**
	 * An array of effects applied to the node.
	 */
	effects: TEffect[];
}

export interface TGeometryMixin {
	geometry?: {
		/**
		 * An array of paths representing the object fill relative to the node.
		 */
		fill: TVectorPath[];
		/**
		 * An array of paths representing the object strokes relative to the node.
		 */
		stroke: TVectorPath[];
	};
}

// =============================================================================
// Effects
// =============================================================================

export interface TDropShadowEffect {
	type: 'DROP_SHADOW';
	color: TRGBA;
	offset: TVector;
	radius: number;
	spread?: number;
	visible: boolean;
	blendMode: TBlendMode;
	showShadowBehindNode?: boolean;
}

export interface TInnerShadowEffect {
	type: 'INNER_SHADOW';
	color: TRGBA;
	offset: TVector;
	radius: number;
	spread?: number;
	visible: boolean;
	blendMode: TBlendMode;
}

export interface TBlurEffect {
	type: 'LAYER_BLUR' | 'BACKGROUND_BLUR';
	radius: number;
	visible: boolean;
}

export type TBlendMode =
	| 'PASS_THROUGH'
	| 'NORMAL'
	| 'DARKEN'
	| 'MULTIPLY'
	| 'LINEAR_BURN'
	| 'COLOR_BURN'
	| 'LIGHTEN'
	| 'SCREEN'
	| 'LINEAR_DODGE'
	| 'COLOR_DODGE'
	| 'OVERLAY'
	| 'SOFT_LIGHT'
	| 'HARD_LIGHT'
	| 'DIFFERENCE'
	| 'EXCLUSION'
	| 'HUE'
	| 'SATURATION'
	| 'COLOR'
	| 'LUMINOSITY';

export type TEffect = TDropShadowEffect | TInnerShadowEffect | TBlurEffect;

// =============================================================================
// Paints
// =============================================================================

export interface TBasePaintMixin {
	opacity: number;
	blendMode: TBlendMode;
	isVisible: boolean;
}

export type TSolidPaint = {
	type: 'SOLID';
	color: TRGB;
} & TBasePaintMixin;

export type TGradientPaint = TGradientPaintExported | TGradientPaintInline;

export type TGradientPaintInline =
	| TLinearGradientPaintInline
	| TRadialGradientPaintInline
	| TAngularGradientPaintInline
	| TDiamondGradientPaintInline;

export type TLinearGradientPaintInline = {
	type: 'GRADIENT_LINEAR';
	isExported: false;
	transform: TTransform;
	gradientStops: TColorStop[];
} & TBasePaintMixin;

export type TRadialGradientPaintInline = {
	type: 'GRADIENT_RADIAL';
	isExported: false;
	transform: TTransform;
	gradientStops: TColorStop[];
} & TBasePaintMixin;

export type TAngularGradientPaintInline = {
	type: 'GRADIENT_ANGULAR';
	isExported: false;
	transform: TTransform;
	gradientStops: TColorStop[];
} & TBasePaintMixin;

export type TDiamondGradientPaintInline = {
	type: 'GRADIENT_DIAMOND';
	isExported: false;
	transform: TTransform;
	gradientStops: TColorStop[];
} & TBasePaintMixin;

export type TGradientPaintExported = {
	type: 'GRADIENT_LINEAR' | 'GRADIENT_RADIAL' | 'GRADIENT_ANGULAR' | 'GRADIENT_DIAMOND';
	isExported: true;
	format: 'JPG' | 'PNG' | 'SVG';
	/**
	 * The hash of the exported gradient file. Used to identify the file.
	 */
	hash: string;
	/**
	 * Optional content of the exported gradient file.
	 * It can be either an array of bytes that contains the exported file's data inline,
	 * or a URL string pointing to the file location.
	 *
	 * If not set the content can to be searched by the hash.
	 */
	content?: Uint8Array | string;
} & TBasePaintMixin;

export type TImagePaint = TImagePaintFill | TImagePaintFit | TImagePaintCrop | TImagePaintTile;

export type TImagePaintFill = {
	type: 'IMAGE';
	scaleMode: 'FILL';
	rotation: number;
} & TBaseImagePaintMixin;

export type TImagePaintFit = {
	type: 'IMAGE';
	scaleMode: 'FIT';
	rotation: number;
} & TBaseImagePaintMixin;

export type TImagePaintCrop = {
	type: 'IMAGE';
	scaleMode: 'CROP';
	transform: TTransform;
} & TBaseImagePaintMixin;

export type TImagePaintTile = {
	type: 'IMAGE';
	scaleMode: 'TILE';
	rotation: number;
	scalingFactor: number;
} & TBaseImagePaintMixin;

export type TBaseImagePaintMixin = {
	/**
	 * The hash of the image file. Used to identify the file.
	 */
	hash: string;
	/**
	 * Optional content of the image file.
	 * It can be either an array of bytes that contains the exported file's data inline,
	 * or a URL string pointing to the file location.
	 *
	 * If not set the content can to be searched by the hash.
	 */
	content?: Uint8Array | string;
	filters?: TImageFilters;
	width: number;
	height: number;
} & TBasePaintMixin;

export interface TImageFilters {
	exposure?: number;
	contrast?: number;
	saturation?: number;
	temperature?: number;
	tint?: number;
	highlights?: number;
	shadows?: number;
}

export interface TColorStop {
	position: number;
	color: TRGBA;
}

export type TEmbedPaint = {
	type: 'EMBED';
	embedData: TEmbedMetaData;
} & TBasePaintMixin;

export type TPaint = TSolidPaint | TGradientPaint | TImagePaint | TEmbedPaint;

// =============================================================================
// Base
// =============================================================================

export interface TVector {
	x: number;
	y: number;
}

export interface TVectorPath {
	/**
	 * The winding rule for the path (same as in SVGs).
	 * This determines whether a given point in space is inside or outside the path.
	 */
	windingRule: WindingRule;
	/**
	 * Data of the vector path.
	 *
	 * e.g. 'M 0 100 L 100 100 L 50 0 Z'
	 */
	data: string;
}

type WindingRule = 'NONZERO' | 'EVENODD' | 'NONE';

export type TRGBA = {
	a: number;
} & TRGB;

export interface TRGB {
	r: number;
	g: number;
	b: number;
}

export type TTransform = [
	[number, number, number],
	[number, number, number],
	[number, number, number]
];

// =============================================================================
// Font
// =============================================================================

export interface TTypeface {
	/**
	 * The family of the font (e.g. "Roboto").
	 */
	family: string;
	/**
	 * The name of the style displayed in UI.
	 */
	name: string;
	/**
	 * The style of the font.
	 */
	style: 'regular' | 'italic';
	/**
	 * The weight of the font (e.g. 400 for "Regular", 700 for "Bold").
	 */
	weight: number;
	/**
	 * The hash of the font file. Used to identify the file.
	 */
	hash?: string;
	/**
	 * Optional content of the font file.
	 * It can be either an array of bytes that contains the exported file's data inline,
	 * or a URL string pointing to the file location.
	 *
	 * If not set the content can to be searched by the hash.
	 */
	content?: Uint8Array | string;
}

export type TLetterSpacing =
	| {
			value: number;
			unit: 'PIXELS' | 'PERCENT';
	  }
	| {
			unit: 'AUTO';
	  };

export type TLineHeight =
	| {
			value: number;
			unit: 'PIXELS' | 'PERCENT';
	  }
	| {
			unit: 'AUTO';
	  };

// =============================================================================
// Other
// =============================================================================

export interface TEllipseArcData {
	startingAngle: number;
	endingAngle: number;
	/**
	 * Ratio of the inner radius to the outer radius.
	 */
	innerRadiusRatio: number;
}

export interface TEmbedMetaData {
	/**
	 * The srcUrl of an embed is the URL that will be loaded in an iFrame
	 * when the embed is activated.
	 */
	srcUrl: string;
	/**
	 * The canonicalUrl of an embed is the URL that will be navigated to
	 * when the embed is opened in an external tab.
	 */
	canonicalUrl: string;
	/**
	 * The name of the provider of an embed.
	 */
	title?: string;
	/**
	 * The name of the provider of an embed.
	 *
	 * e.g. 'Spotify', 'YouTube'
	 */
	provider?: string;
}

export interface TSVGElement {
	/**
	 * The name of the SVG element.
	 *
	 * e.g. 'rect', 'circle', 'text'
	 */
	type: string;
	/**
	 * Value of the SVG element, if it requires one.
	 *
	 * e.g. 'Jeff' for a 'text' SVG element
	 */
	value?: string;
	/**
	 * Attributes of the SVG element.
	 *
	 * e.g. For a 'circle' node something like 'cx', 'cy', 'r'
	 */
	attributes: Record<string, string>;
	/**
	 * The list of children, sorted back-to-front.
	 * That is, the first child in the array is the bottommost layer in the SVG,
	 * and the last child in the array is the topmost layer.
	 */
	children: TSVGElement[];
}

type TConstraintType = 'MIN' | 'CENTER' | 'MAX' | 'STRETCH' | 'SCALE';
