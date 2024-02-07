// This file has been generated by Specta. DO NOT EDIT.

/**
 * Represents the absolute position and orientation of a node within the composition's coordinate system.
 * This mixin stores the transformation as a 3x3 matrix (Mat3),
 * which includes translation, rotation, and skew.
 * 
 * Note: This transformation does not include scaling.
 * For scaling, refer to the `DimensionMixin`.
 */
export type AbsoluteTransformMixin = Mat3

/**
 * Represents an anchor point in a path, defining a specific location and command.
 */
export type Anchor = { 
/**
 * The command associated with the anchor point,
 * defining how the path should proceed from this point.
 */
command: AnchorCommand }

export type AnchorCommand = 
/**
 * Moves the path to a new location without drawing anything.
 */
{ type: "MoveTo"; position: Vec2 } | 
/**
 * Draws a straight line from the current position to the anchor point.
 */
{ type: "LineTo"; position: Vec2 } | 
/**
 * Draws a curve to the anchor point using two control points.
 */
{ type: "CurveTo"; position: Vec2; controlPoint1: Vec2; controlPoint2: Vec2 } | 
/**
 * Draws an arc to the anchor point.
 */
{ type: "ArcTo"; position: Vec2; radius: Vec2; xAxisRotation: number; largeArcFlag: boolean; sweepFlag: boolean } | 
/**
 * Closes the path by drawing a line to the start point.
 */
{ type: "ClosePath" }

export type AnyCoreInputEvent = { events: CoreInputEvent[] }

export type AnyInputEvent = ({ type: "Core" } & AnyCoreInputEvent) | ({ type: "Interaction" } & AnyInteractionInputEvent)

export type AnyInteractionInputEvent = { events: InteractionInputEvent[] }

/**
 * Emitted when an attribute of a SVGElement is removed.
 */
export type AttributeRemoved = { key: string }

/**
 * Emitted when an attribute of an SVGElement is updated.
 */
export type AttributeUpdated = { newValue: SVGAttribute }

/**
 * Defines blending properties for a node.
 */
export type BlendMixin = { 
/**
 * Specifies the blend mode for the node.
 * Blend mode determines how the node's color blends with colors beneath it.
 */
blendMode?: BlendMode; 
/**
 * The opacity of the node,
 * ranging from 0.0 (completely transparent) to 1.0 (completely opaque).
 */
opacity?: number; 
/**
 * Indicates whether the node is used as a mask.
 */
isMask?: boolean }

export type BlendMode = "Normal" | "Multiply" | "Screen" | "Overlay" | "Darken" | "Lighten" | "ColorDodge" | "ColorBurn" | "HardLight" | "SoftLight" | "Difference" | "Exclusion" | "Hue" | "Saturation" | "Color" | "Luminosity"

/**
 * Defines how text should break lines within its container.
 */
export type BreakLineOn = 
/**
 * Breaks lines at word boundaries using the Unicode Line Breaking Algorithm.
 */
"WordBoundary" | 
/**
 * Breaks lines at any character, possibly splitting words.
 */
"AnyCharacter" | 
/**
 * Disables automatic line breaking. Respects explicit line breaks like '\n'.
 */
"NoWrap"

/**
 * Manages the child entities of a node in a hierarchical structure.
 * 
 * The children are sorted back-to-front,
 * meaning the first child in the vector is the bottommost layer in the scene,
 * and the last child is the topmost layer.
 */
export type ChildrenMixin = Entity[]

export type ColorStop = { 
/**
 * The position of the color stop in the gradient, ranging from 0.0 to 1.0.
 */
position: number; 
/**
 * The color of the stop, represented as an RGB array.
 */
color: [number, number, number] }

export type CompositionChange = { rootNode: Entity; viewBox: ViewBox; width: number; height: number }

export type CompositionChangeEvent = { change: CompositionChange }

export type CompositionResized = { width: number; height: number }

export type CompositionViewBoxChanged = { viewBox: ViewBox }

export type ContentType = "JPEG" | "PNG" | "SVG"

export type ContinuousId = number

export type CoreInputEvent = ({ type: "EntityMoved" } & EntityMoved) | ({ type: "EntitySetPosition" } & EntitySetPosition) | ({ type: "EntityDeleted" } & EntityDeleted) | ({ type: "NodeCreated" } & NodeCreated) | ({ type: "CompositionResized" } & CompositionResized) | ({ type: "CompositionViewBoxChanged" } & CompositionViewBoxChanged)

export type CursorChangeEvent = { cursor: CursorForFrontend }

export type CursorDownOnComposition = { position: Vec2; button: MouseButton }

export type CursorDownOnEntity = { entity: Entity; position: Vec2; button: MouseButton }

export type CursorDownOnResizeHandle = { initialBounds: XYWH; corner: number; rotationInRadians: number }

export type CursorDownOnRotateHandle = { corner: number; initialRotationInRadians: number }

export type CursorEnteredComposition = null

export type CursorExitedComposition = null

export type CursorForFrontend = { type: "Default" } | { type: "Resize"; rotationInDegrees: number } | { type: "Rotate"; rotationInDegrees: number }

export type CursorMovedOnComposition = { position: Vec2 }

export type CursorUpOnComposition = { position: Vec2; button: MouseButton }

/**
 * Represents the composition in which all nodes exist.
 */
export type DTIFComposition = { 
/**
 * The version of the composition type declaration, used internally.
 * Defaults to the latest version.
 */
version?: string; 
/**
 * The name of the composition.
 * Example: 'My super cool composition'.
 */
name: string; 
/**
 * The width of the composition, in units.
 */
width: number; 
/**
 * The height of the composition, in units.
 */
height: number; viewBox?: ViewBox | null; 
/**
 * The identifier of the root node in the composition.
 */
rootNodeId: Entity; 
/**
 * A mapping of node identifiers to their corresponding nodes within the composition.
 * Note: Planned to directly use `Entity` as a key once the referenced serde issue is resolved.
 * https://github.com/serde-rs/serde/issues/1183
 */
nodes: { [key in string]: NodeBundle }; 
/**
 * A mapping of paint identifiers to their corresponding paints within the composition.
 * Note: Planned to directly use `Entity` as a key once the referenced serde issue is resolved.
 * https://github.com/serde-rs/serde/issues/1183
 */
paints: { [key in string]: PaintBundle }; 
/**
 * A mapping of font identifiers to their corresponding font data within the composition.
 * Note: Planned to directly use `u64` as a key once the referenced serde issue is resolved.
 * https://github.com/serde-rs/serde/issues/1183
 */
fonts?: FontContent[]; 
/**
 * Optional list of changes represented as core input events.
 * This field is optional and defaults to `None` if not provided.
 */
changes?: CoreInputEvent[] | null }

/**
 * Represents the dimensional properties of a node, specifically its width and height.
 */
export type DimensionMixin = { 
/**
 * The width of the node, measured in units (likely pixels).
 * This field specifies the horizontal dimension of the node.
 */
width: number; 
/**
 * The height of the node, measured in units (likely pixels).
 * This field determines the vertical dimension of the node.
 */
height: number }

/**
 * Emitted when a SVGElement (child) is append to another SVGElement (parent).
 */
export type ElementAppended = { parentId: ContinuousId }

export type ElementChange = ({ type: "ElementCreated" } & ElementCreated) | ({ type: "ElementDeleted" }) | ({ type: "ElementAppended" } & ElementAppended) | ({ type: "AttributeUpdated" } & AttributeUpdated) | ({ type: "AttributeRemoved" } & AttributeRemoved) | ({ type: "StyleUpdated" } & StyleUpdated) | ({ type: "StyleRemoved" } & StyleRemoved)

export type ElementChangeEvent = { id: ContinuousId; changes: ElementChange[] }

/**
 * Emitted when a new SVGElement is created.
 */
export type ElementCreated = { tagName: string; attributes: SVGAttribute[]; styles: SVGStyle[]; parentId: ContinuousId | null; isBundleRoot: boolean; entity: Entity | null }

/**
 * Emitted when a SVGElement is deleted.
 */
export type ElementDeleted = Record<string, never>

/**
 * Represents the arc data for an ellipse.
 * This includes properties for defining the sweep of the ellipse and its inner radius,
 * which are used in UI elements to create various elliptical shapes.
 */
export type EllipseArcData = { 
/**
 * The starting angle of the ellipse's arc.
 */
startingAngle: number; 
/**
 * The ending angle of the ellipse's arc.
 */
endingAngle: number; 
/**
 * The ratio of the inner radius to the outer radius of the ellipse.
 * A value of 0 indicates a full ellipse, while higher values create a 'donut' shape.
 */
innerRadiusRatio: number }

/**
 * Represents a basic shape node for an ellipse.
 * Note that a circle is a special case of an ellipse where the width equals the height.
 */
export type EllipseNode = { _ellipse_node?: null | null; 
/**
 * Contains the arc data for the ellipse,
 * which includes the starting angle, ending angle, and the inner radius ratio.
 * These properties are used to create arcs and donuts shapes.
 */
arcData?: EllipseArcData }

export type EllipseNodeBundle = ({ _ellipse_node?: null | null; 
/**
 * Contains the arc data for the ellipse,
 * which includes the starting angle, ending angle, and the inner radius ratio.
 * These properties are used to create arcs and donuts shapes.
 */
arcData?: EllipseArcData }) & ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & { node?: Node; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin; fill?: FillMixin }

export type Entity = number

export type EntityDeleted = { entity: Entity }

export type EntityMoved = { entity: Entity; dx: number; dy: number }

export type EntitySetPosition = { entity: Entity; x: number; y: number }

/**
 * Manages the fill properties of a graphical object.
 */
export type FillMixin = { 
/**
 * A collection of `Paint` objects,
 * each defining a different aspect of how the object is filled.
 */
paintIds?: Entity[] }

/**
 * Defines the content of a font.
 */
export type FontContent = 
/**
 * Font content stored as binary data.
 */
{ type: "Binary"; content: number[] } | 
/**
 * Font content referenced by a URL.
 * 
 * This variant is only supported when the `resolve-url` feature is enabled.
 */
{ type: "Url"; url: string }

export type FontMetadata = { 
/**
 * The font family to which this font belongs.
 */
family: string; 
/**
 * The style of the font, such as italic or normal.
 */
style: FontStyle; 
/**
 * The weight of the font, typically ranging from 100 (thin) to 900 (black).
 */
weight: number }

/**
 * A font style property.
 */
export type FontStyle = 
/**
 * A face that is neither italic not obliqued.
 */
"Normal" | 
/**
 * A form that is generally cursive in nature.
 */
"Italic" | 
/**
 * A typically-sloped version of the regular face.
 */
"Oblique"

/**
 * Acts as a container used to define a layout hierarchy.
 * It functions similarly to an HTML `<div>` element.
 * This is distinct from a `GroupNode`, which is more akin to a folder for layers in its use and functionality.
 */
export type FrameNode = { _frame_node?: null | null; 
/**
 * Indicates whether the frame clips its content to its bounding box.
 * When set to `true`, content that extends beyond the frame's boundaries will be clipped.
 * When `false`, content can extend beyond the frame's boundaries without being clipped.
 */
clipContent?: boolean }

export type FrameNodeBundle = ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & ({ _frame_node?: null | null; 
/**
 * Indicates whether the frame clips its content to its bounding box.
 * When set to `true`, content that extends beyond the frame's boundaries will be clipped.
 * When `false`, content can extend beyond the frame's boundaries without being clipped.
 */
clipContent?: boolean }) & { node?: Node; rectangleCornerMixin?: RectangleCornerMixin; children: ChildrenMixin; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin; fill?: FillMixin }

export type GradientPaint = { _gradient_paint?: null | null; 
/**
 * Specifies the variant of the gradient.
 */
variant?: GradientPaintVariant }

export type GradientPaintBundle = ({ _gradient_paint?: null | null; 
/**
 * Specifies the variant of the gradient.
 */
variant?: GradientPaintVariant }) & ({ 
/**
 * A list of color stops defining the gradient.
 */
gradientStops: ColorStop[] }) & { paint?: Paint; compositionMixin?: PaintCompositionMixin; blendMixin?: BlendMixin }

export type GradientPaintVariant = { type: "Linear"; transform?: Mat3 } | { type: "Radial"; transform?: Mat3 }

export type GradientStopsMixin = { 
/**
 * A list of color stops defining the gradient.
 */
gradientStops: ColorStop[] }

/**
 * Serves as a container used to semantically group related nodes,
 * analogous to a folder in a layers panel.
 * This is in contrast to the `Frame` node, which is used to define layout and is
 * more akin to an HTML `<div>` element.
 * 
 * Groups are automatically positioned and sized to accommodate their content.
 * As a result, while it is possible to move or resize a `Group`, be aware that its
 * position and size are subject to change in response to modifications of its content.
 */
export type GroupNode = { _group_node?: null | null }

export type GroupNodeBundle = ({ _group_node?: null | null }) & ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & { node?: Node; children: ChildrenMixin; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin }

export type HandleSide = "Top" | "Bottom" | "Left" | "Right"

/**
 * Horizontal alignment options for text within its container.
 */
export type HorizontalTextAlignment = 
/**
 * Aligns text to the left side of its container.
 */
"Left" | 
/**
 * Centers text horizontally within its container.
 */
"Center" | 
/**
 * Aligns text to the right side of its container.
 */
"Right" | 
/**
 * Justifies text across the container width.
 */
"Justified"

export type ImageContent = 
/**
 * Image content stored as binary data.
 */
{ type: "Binary"; content: number[]; contentType: ContentType } | 
/**
 * Image content referenced by a URL.
 * 
 * This variant is only supported when the `resolve-url` feature is enabled.
 */
{ type: "Url"; url: string; contentType: ContentType }

export type ImageContentMixin = { 
/**
 * The width of the image in pixels.
 */
width: number; 
/**
 * The height of the image in pixels.
 */
height: number; 
/**
 * The actual content of the image.
 */
content: ImageContent }

export type ImagePaint = { _image_paint?: null | null; 
/**
 * Defines the scale mode of the image.
 */
scaleMode?: ImagePaintScaleMode }

export type ImagePaintBundle = ({ _image_paint?: null | null; 
/**
 * Defines the scale mode of the image.
 */
scaleMode?: ImagePaintScaleMode }) & { paint?: Paint; imageContent: ImageContentMixin; compositionMixin?: PaintCompositionMixin; blendMixin?: BlendMixin }

export type ImagePaintScaleMode = 
/**
 * Fills the area completely with the image.
 */
{ type: "Fill"; _image_fill_paint?: null | null } | 
/**
 * Fits the image within the area while maintaining its aspect ratio.
 */
{ type: "Fit"; _image_fit_paint?: null | null } | 
/**
 * Crops the image to fill the area.
 */
{ type: "Crop"; _image_crop_paint?: null | null; transform?: Mat3 } | 
/**
 * Tiles the image within the area.
 */
{ type: "Tile"; imageTilePaint?: null | null; rotation?: number; scalingFactor: number }

export type InteractionInputEvent = ({ type: "CursorDownOnEntity" } & CursorDownOnEntity) | ({ type: "CursorMovedOnComposition" } & CursorMovedOnComposition) | ({ type: "CursorEnteredComposition" }) | ({ type: "CursorExitedComposition" }) | ({ type: "CursorDownOnComposition" } & CursorDownOnComposition) | ({ type: "CursorUpOnComposition" } & CursorUpOnComposition) | ({ type: "WheeledOnComposition" } & WheeledOnComposition) | ({ type: "CursorDownOnResizeHandle" } & CursorDownOnResizeHandle) | ({ type: "CursorDownOnRotateHandle" } & CursorDownOnRotateHandle)

export type InteractionModeChangeEvent = { interactionMode: InteractionModeForFrontend }

export type InteractionModeForFrontend = { type: "None" } | { type: "Pressing" } | { type: "Translating" } | { type: "Resizing" } | { type: "Rotating" } | { type: "Dragging" }

/**
 * Options for spacing between characters in a text span.
 */
export type LetterSpacing = 
/**
 * Automatic spacing based on font metrics.
 */
"Auto" | 
/**
 * Fixed spacing in pixels.
 */
{ Pixels: number } | 
/**
 * Spacing as a percentage of font size.
 */
{ Percent: number }

/**
 * Options for controlling line height in text.
 */
export type LineHeight = 
/**
 * Automatic line height based on font metrics.
 */
"Auto" | 
/**
 * Fixed line height in pixels.
 */
{ Pixels: number } | 
/**
 * Line height as a percentage of font size.
 */
{ Percent: number }

export type Locked = null

export type Mat3 = [number, number, number, number, number, number, number, number, number]

export type MixinChange = ({ type: "Dimension" } & DimensionMixin) | ({ type: "Blend" } & BlendMixin) | ({ type: "NodeComposition" } & NodeCompositionMixin) | ({ type: "Children" } & MixinChangeChildrenMixin) | ({ type: "RelativeTransform" } & MixinChangeRelativeTransformMixin) | ({ type: "Path" } & PathMixin) | ({ type: "PaintComposition" } & PaintCompositionMixin) | ({ type: "SolidPaint" } & SolidPaint) | ({ type: "ImagePaint" } & SVGImagePaint) | ({ type: "ImageContent" } & ImageContentMixin) | ({ type: "GradientPaint" } & SVGGradientPaint) | ({ type: "GradientStopsMixin" } & GradientStopsMixin)

export type MixinChangeChildrenMixin = { children: ChildrenMixin }

export type MixinChangeRelativeTransformMixin = { relativeTransform: RelativeTransformMixin }

export type MouseButton = "Left" | "Middle" | "Right" | "Unkown"

/**
 * Represents a basic node in the composition.
 */
export type Node = { 
/**
 * Represents the specific type of the node, such as `Rectangle`, `Ellipse`, `Star`, etc.
 * This field is redundant but neccessary to distinguish different nodes in the rendering process,
 * without a big overhead like a separate system for each node type/variant.
 * Note that the NodeType should be equivalent to the 'NodeBundle' enum
 * and when creating a new `NodeBundle` always use the default of that specific bundle!
 */
node_type: NodeType }

export type NodeBundle = ({ type: "Frame" } & FrameNodeBundle) | ({ type: "Group" } & GroupNodeBundle) | ({ type: "Rectangle" } & RectangleNodeBundle) | ({ type: "Text" } & TextNodeBundle) | ({ type: "Vector" } & VectorNodeBundle) | ({ type: "Polygon" } & PolygonNodeBundle) | ({ type: "Ellipse" } & EllipseNodeBundle) | ({ type: "Star" } & StarNodeBundle)

/**
 * Contains properties related to the composition settings of a node.
 */
export type NodeCompositionMixin = { 
/**
 * Determines the visibility of the node.
 */
isVisible?: boolean; 
/**
 * Indicates whether the node is locked or not.
 * A locked node restricts certain user interactions,
 * such as selecting or dragging on the canvas.
 */
isLocked?: boolean }

export type NodeCreated = { parentEntity: Entity | null; node: NodeBundle }

export type NodeMetaMixin = { 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }

export type NodeType = "Group" | "Rectangle" | "Frame" | "Text" | "Vector" | "Polygon" | "Ellipse" | "Star"

export type OutputEvent = ({ type: "ElementChange" } & ElementChangeEvent) | ({ type: "CompositionChange" } & CompositionChangeEvent) | ({ type: "TrackUpdate" } & TrackUpdateEvent) | ({ type: "SelectionChange" } & SelectionChangeEvent) | ({ type: "InteractionModeChange" } & InteractionModeChangeEvent) | ({ type: "CursorChange" } & CursorChangeEvent)

/**
 * Represents a basic paint in the composition.
 */
export type Paint = { 
/**
 * Represents the specific type of the paint, such as `Solid`, `Image`, `Gradient`, etc.
 * This field is redundant but neccessary to distinguish different paints in the rendering process,
 * without a big overhead like a separate system for each paint type/variant.
 * Note that the PaintType should be equivalent to the 'PaintBundle' enum
 * and when creating a new `PaintBundle` always use the default of that specific bundle!
 */
paint_type: PaintType }

export type PaintBundle = ({ type: "Solid" } & SolidPaintBundle) | ({ type: "Image" } & ImagePaintBundle) | ({ type: "Gradient" } & GradientPaintBundle)

/**
 * Contains properties related to the composition settings of a paint.
 */
export type PaintCompositionMixin = { 
/**
 * Determines the visibility of the paint.
 */
isVisible?: boolean }

export type PaintType = "Solid" | "Gradient" | "Image"

/**
 * Represents a path in a graphical composition, defined by a series of vertices.
 * Each vertex is an anchor point, and the path is constructed by connecting these points.
 */
export type PathMixin = { 
/**
 * A collection of `Anchor` points that define the shape of the path.
 * These vertices determine the path's outline through various commands.
 */
vertices: Anchor[] }

/**
 * Represents a basic shape node for a regular convex polygon with three or more sides.
 */
export type PolygonNode = { _polygon_node?: null | null; 
/**
 * The number of sides of the polygon.
 * This value must be an integer greater than or equal to 3.
 */
pointCount?: number }

export type PolygonNodeBundle = ({ _polygon_node?: null | null; 
/**
 * The number of sides of the polygon.
 * This value must be an integer greater than or equal to 3.
 */
pointCount?: number }) & ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & { node?: Node; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin; fill?: FillMixin }

/**
 * Provides corner radius properties for rectangle like nodes.
 */
export type RectangleCornerMixin = { 
/**
 * The radius in pixels for rounding the top left corner of the node.
 * This value determines how curved the top left corner will be.
 */
topLeftRadius?: number; 
/**
 * The radius in pixels for rounding the top right corner of the node.
 * This value influences the curvature of the top right corner.
 */
topRightRadius?: number; 
/**
 * The radius in pixels for rounding the bottom right corner of the node.
 * Adjusts the curve of the bottom right corner.
 */
bottomRightRadius?: number; 
/**
 * The radius in pixels for rounding the bottom left corner of the node.
 * Modifies the roundness of the bottom left corner.
 */
bottomLeftRadius?: number }

/**
 * Represents a basic shape node for a rectangle.
 * It is a fundamental building block used to create and manipulate rectangular shapes
 * within the composition.
 */
export type RectangleNode = { _rectangle_node?: null | null }

export type RectangleNodeBundle = ({ _rectangle_node?: null | null }) & ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & { node?: Node; rectangleCornerMixin?: RectangleCornerMixin; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin; fill?: FillMixin }

/**
 * Represents the relative position and orientation of a node within its parent's coordinate system.
 * This mixin stores the transformation as a 3x3 matrix (Mat3),
 * which includes translation, rotation, and skew.
 * 
 * Note: This transformation does not include scaling.
 * For scaling, refer to the `DimensionMixin`.
 */
export type RelativeTransformMixin = Mat3

/**
 * Marks the root node within the composition or scene.
 * 
 * This component is intended to be used with only one entity in the world
 * to represent the starting point of the composition.
 * It is important to note that there is no automatic enforcement
 * at the Bevy framework level to ensure the uniqueness of this component.
 * As such, maintaining the singularity of this component must be managed
 * through game logic or specific programming measures to prevent multiple instances.
 */
export type Root = null

export type SVGAttribute = { type: "Id"; id: ContinuousId } | { type: "Width"; width: number; unit: SVGMeasurementUnit } | { type: "Height"; height: number; unit: SVGMeasurementUnit } | { type: "Opacity"; opacity: number } | { type: "Transform"; transform: SVGTransformAttribute } | { type: "PatternTransform"; transform: SVGTransformAttribute } | { type: "D"; d: SVGDAttribute } | { type: "ClipPath"; clipPath: ContinuousId } | { type: "Fill"; fill: string } | { type: "ReferencedFill"; id: ContinuousId } | { type: "Name"; name: string } | { type: "PatternUnits"; patternUnits: SVGUnitsVariant } | { type: "GradientUnits"; gradientUnits: SVGUnitsVariant } | { type: "Href"; href: SVGHrefVariant } | { type: "PreserveAspectRatio"; preserveAspectRatio: string } | { type: "X1"; x1: number } | { type: "Y1"; y1: number } | { type: "X2"; x2: number } | { type: "Y2"; y2: number } | { type: "Offset"; offset: number } | { type: "StopColor"; stopColor: string } | { type: "PointerEvents"; pointerEvents: SVGPointerEventsVariants }

export type SVGBlendMode = { type: "Normal" } | { type: "Multiply" } | { type: "Screen" } | { type: "Overlay" } | { type: "Darken" } | { type: "Lighten" } | { type: "ColorDodge" } | { type: "ColorBurn" } | { type: "HardLight" } | { type: "SoftLight" } | { type: "Difference" } | { type: "Exclusion" } | { type: "Hue" } | { type: "Saturation" } | { type: "Color" } | { type: "Luminosity" }

export type SVGDAttribute = { type: "Meta"; value: SVGPathCommand[] } | { type: "String"; value: string }

export type SVGDisplayStyle = { type: "Block" } | { type: "None" }

export type SVGGradientPaint = { variant: SVGGradientPaintVariant }

export type SVGGradientPaintVariant = { Linear: { start: Vec2; end: Vec2 } } | { Radial: { center: Vec2; radius: Vec2; rotation: number } }

export type SVGHrefVariant = { type: "Base64"; content: string; contentType: ContentType } | { type: "Url"; url: string }

export type SVGImagePaint = { scale_mode: SVGImagePaintScaleMode }

export type SVGImagePaintScaleMode = "Fill" | "Fit" | { Crop: { transform: Mat3; image_width: number; image_height: number } } | { Tile: { rotation: number; tile_width: number; tile_height: number } }

export type SVGMeasurementUnit = { type: "Pixel" } | { type: "Percent" }

export type SVGPathCommand = { type: "MoveTo"; x: number; y: number } | { type: "LineTo"; x: number; y: number } | { type: "CurveTo"; cx1: number; cy1: number; cx2: number; cy2: number; x: number; y: number } | { type: "ArcTo"; rx: number; ry: number; xAxisRotation: number; largeArcFlag: boolean; sweepFlag: boolean; x: number; y: number } | { type: "ClosePath" }

export type SVGPointerEventsVariants = { type: "None" } | { type: "All" }

export type SVGRenderOutputEvent = ({ type: "ElementChange" } & ElementChangeEvent)

export type SVGStyle = { type: "Display"; display: SVGDisplayStyle } | { type: "BlendMode"; blendMode: SVGBlendMode }

export type SVGTransformAttribute = { type: "Matrix"; a: number; b: number; c: number; d: number; tx: number; ty: number } | { type: "Rotate"; rotation: number }

export type SVGUnitsVariant = { type: "UserSpaceOnUse" } | { type: "ObjectBoundingBox" }

export type Selected = null

export type SelectionChangeEvent = { selected: Entity[] }

export type SolidPaint = { _solid_paint?: null | null; 
/**
 * The color of the paint, represented as an RGB array
 * where each component ranges from 0 to 255.
 */
color: [number, number, number] }

export type SolidPaintBundle = ({ _solid_paint?: null | null; 
/**
 * The color of the paint, represented as an RGB array
 * where each component ranges from 0 to 255.
 */
color: [number, number, number] }) & { paint?: Paint; compositionMixin?: PaintCompositionMixin; blendMixin?: BlendMixin }

/**
 * Represents a basic shape node for a star with a set number of points.
 */
export type StarNode = { _star_node?: null | null; 
/**
 * The number of "spikes", or outer points of the star.
 * This value must be an integer greater than or equal to 3.
 */
pointCount?: number; 
/**
 * The ratio of the inner radius to the outer radius of the star.
 * This value is used to define the sharpness of the star's points.
 */
innerRadiusRatio: number }

export type StarNodeBundle = ({ _star_node?: null | null; 
/**
 * The number of "spikes", or outer points of the star.
 * This value must be an integer greater than or equal to 3.
 */
pointCount?: number; 
/**
 * The ratio of the inner radius to the outer radius of the star.
 * This value is used to define the sharpness of the star's points.
 */
innerRadiusRatio: number }) & ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & { node?: Node; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin; fill?: FillMixin }

/**
 * Emitted when a style property of a SVGElement is removed.
 */
export type StyleRemoved = { key: string }

/**
 * Emitted when a style property of a SVGElement is updated.
 */
export type StyleUpdated = { newValue: SVGStyle }

/**
 * Represents a text node with customizable style and layout properties.
 */
export type TextNode = { _text_node?: null | null; 
/**
 * Sections of the text, each with its own style.
 */
spans: TextSpan[]; 
/**
 * Horizontal alignment of the text within its container.
 */
horizontalTextAlignment?: HorizontalTextAlignment; 
/**
 * Vertical alignment of the text within its container.
 */
verticalTextAlignment?: VerticalTextAlignment; 
/**
 * Behavior of text line breaking at the bounds of its container.
 */
linebreakBehavior?: BreakLineOn }

export type TextNodeBundle = ({ _text_node?: null | null; 
/**
 * Sections of the text, each with its own style.
 */
spans: TextSpan[]; 
/**
 * Horizontal alignment of the text within its container.
 */
horizontalTextAlignment?: HorizontalTextAlignment; 
/**
 * Vertical alignment of the text within its container.
 */
verticalTextAlignment?: VerticalTextAlignment; 
/**
 * Behavior of text line breaking at the bounds of its container.
 */
linebreakBehavior?: BreakLineOn }) & ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & { node?: Node; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin; fill?: FillMixin }

/**
 * A span of text with a specific style.
 */
export type TextSpan = { 
/**
 * Text content of the span.
 */
text: string; 
/**
 * Font metadata to identify font
 */
font: FontMetadata; 
/**
 * Style properties applied to this span.
 */
style: TextStyle }

/**
 * Style properties for a text span, defining its appearance.
 */
export type TextStyle = { 
/**
 * Height of rasterized glyphs in pixels, influenced by window scale.
 */
fontSize: number; 
/**
 * Spacing between characters.
 */
letterSpacing?: LetterSpacing; 
/**
 * Vertical spacing between lines of text.
 */
lineHeight?: LineHeight }

export type TrackUpdateEvent = { id: Entity; updates: MixinChange[] }

export type TrackableMixinType = { type: "Dimension" } | { type: "RelativeTransform" }

export type Vec2 = [number, number]

/**
 * Represents a basic vector. It is the most general representation of a shape.
 */
export type VectorNode = { _vector_node?: null | null }

export type VectorNodeBundle = ({ _vector_node?: null | null }) & ({ 
/**
 * The name of the node.
 * This is an optional field and can be used to label the node with a descriptive name,
 * such as 'Cool Node'.
 * If not provided, it defaults to `None`.
 */
name?: string | null }) & ({ 
/**
 * A collection of `Anchor` points that define the shape of the path.
 * These vertices determine the path's outline through various commands.
 */
vertices: Anchor[] }) & { node?: Node; compositionMixin?: NodeCompositionMixin; relativeTransform: RelativeTransformMixin; dimension: DimensionMixin; blendMixin?: BlendMixin; fill?: FillMixin }

/**
 * Vertical alignment options for text within its container.
 */
export type VerticalTextAlignment = 
/**
 * Aligns text to the top of its container.
 */
"Top" | 
/**
 * Centers text vertically within its container.
 */
"Center" | 
/**
 * Aligns text to the bottom of its container.
 */
"Bottom"

export type ViewBox = { width: number; height: number; minX: number; minY: number }

export type WheeledOnComposition = { position: Vec2; delta: Vec2; ctrlKeyPressed: boolean; metaKeyPressed: boolean }

export type XYWH = { position: Vec2; width: number; height: number }

