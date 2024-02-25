 // This file has been generated by Specta. DO NOT EDIT.

export type BlendMode = "Normal" | "Multiply" | "Screen" | "Overlay" | "Darken" | "Lighten" | "ColorDodge" | "ColorBurn" | "HardLight" | "SoftLight" | "Difference" | "Exclusion" | "Hue" | "Saturation" | "Color" | "Luminosity"

export type BreakLineOn = "WordBoundary" | "AnyCharacter" | "NoWrap"

export type Centimeter = number

export type Color = { red: number; green: number; blue: number }

export type CompCoreInputEvent = ({ type: "CompositionResized" } & CompositionResizedInputEvent) | ({ type: "CompositionViewportChanged" } & CompositionViewportChangedInputEvent) | ({ type: "EntityMoved" } & EntityMovedInputEvent) | ({ type: "EntitySetPosition" } & EntitySetPositionInputEvent) | ({ type: "EntityDeleted" } & EntityDeletedInputEvent)

export type CompDtif = { 
/**
 * The version of the composition type declaration.
 */
version: string; 
/**
 * The name of the composition.
 */
name: string; 
/**
 * The size of the composition in pixels.
 */
size: Size; 
/**
 * The viewport defines the area on the render target to which the camera renders its image.
 */
viewport?: Viewport; 
/**
 * The identifier of the root node in the composition.
 */
rootNodeId: string; 
/**
 * A mapping of node identifiers to their corresponding nodes within the composition.
 */
nodes: { [key in string]: Node }; 
/**
 * A mapping of image identifiers to their corresponding images within the composition.
 */
images: { [key in string]: Content }; 
/**
 * A list of font data.
 */
fonts?: Content[]; events?: DtifInputEvent[] }

export type CompositionResizedInputEvent = { size: Size }

export type CompositionViewportChangedInputEvent = { viewport: Viewport }

export type Content = 
/**
 * Content stored as binary data.
 */
{ type: "Binary"; content: number[] } | 
/**
 * Content referenced by a URL.
 */
{ type: "Url"; url: string }

export type CornerRadii = Vec4

export type CursorDownOnCompInputEvent = { position: Vec2; button: MouseButton }

export type CursorDownOnEntityInputEvent = { entity: Entity; position: Vec2; button: MouseButton }

export type CursorDownOnResizeHandleInputEvent = { initialBounds: XYWH; corner: number; rotationInRadians: number }

export type CursorDownOnRotateHandleInputEvent = { corner: number; initialRotationInRadians: number }

export type CursorEnteredCompInputEvent = null

export type CursorExitedCompInputEvent = null

export type CursorMovedOnCompInputEvent = { position: Vec2 }

export type CursorUpOnCompInputEvent = { position: Vec2; button: MouseButton }

export type DtifCompositionResizedEvent = { size: Size }

export type DtifCompositionViewportChangedEvent = { viewport: Viewport }

export type DtifEntityDeletedEvent = { entity: string }

export type DtifEntityMovedEvent = { entity: string; dx: number; dy: number }

export type DtifEntitySetPositionEvent = { entity: string; x: number; y: number }

export type DtifInputEvent = ({ type: "CompositionResized" } & DtifCompositionResizedEvent) | ({ type: "CompositionViewportChanged" } & DtifCompositionViewportChangedEvent) | ({ type: "EntityMoved" } & DtifEntityMovedEvent) | ({ type: "EntitySetPosition" } & DtifEntitySetPositionEvent) | ({ type: "EntityDeleted" } & DtifEntityDeletedEvent)

export type Entity = number

export type EntityDeletedInputEvent = { entity: Entity }

export type EntityMovedInputEvent = { entity: Entity; dx: number; dy: number }

export type EntitySetPositionInputEvent = { entity: Entity; x: number; y: number }

/**
 * Describes a fill style, including paint type, blend mode, and opacity.
 */
export type Fill = { paint: Paint; blendMode: BlendMode; opacity: Opacity }

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

export type FrameNode = { clipContent: boolean; translation: Vec2; angleInRadians: number; size: Size; cornerRadii: CornerRadii; visibility: Visibility; fill: Fill[]; blendMode: BlendMode; opacity: Opacity; stroke: Stroke[]; children: string[] }

export type GroupNode = { translation: Vec2; angleInRadians: number; size: Size; visibility: Visibility; blendMode: BlendMode; opacity: Opacity; children: string[] }

export type HandleSide = "Top" | "Bottom" | "Left" | "Right"

export type HorizontalTextAlignment = "Left" | "Center" | "Right" | "Justified"

export type Inch = number

export type InteractionInputEvent = ({ type: "CursorDownOnEntity" } & CursorDownOnEntityInputEvent) | ({ type: "CursorMovedOnComposition" } & CursorMovedOnCompInputEvent) | ({ type: "CursorEnteredComposition" }) | ({ type: "CursorExitedComposition" }) | ({ type: "CursorDownOnComposition" } & CursorDownOnCompInputEvent) | ({ type: "CursorUpOnComposition" } & CursorUpOnCompInputEvent) | ({ type: "WheeledOnComposition" } & WheeledOnCompInputEvent) | ({ type: "CursorDownOnResizeHandle" } & CursorDownOnResizeHandleInputEvent) | ({ type: "CursorDownOnRotateHandle" } & CursorDownOnRotateHandleInputEvent)

export type InteractionMode = 
/**
 * Default canvas mode. Nothing is happening.
 */
{ type: "None" } | 
/**
 * When the user's pointer is pressed.
 */
{ type: "Pressing"; origin: Vec2; button: MouseButton } | 
/**
 * When the user is dragging.
 */
{ type: "Dragging"; current: Vec2 } | 
/**
 * When the user is moving selected nodes.
 */
{ type: "Translating"; origin: Vec2; current: Vec2 } | 
/**
 * When the user is resizing the selected nodes.
 */
{ type: "Resizing"; corner: number; initial_bounds: XYWH; rotation_in_degrees: number } | 
/**
 * When the user is rotating the selected nodes.
 */
{ type: "Rotating"; corner: number; initial_rotation_in_radians: number; rotation_in_degrees: number }

export type LetterSpacing = "Auto" | { Fixed: MeasurementUnit }

export type LineHeight = "Auto" | { Fixed: MeasurementUnit }

export type MeasurementUnit = { type: "Pixels"; pixels: Pixel } | { type: "Percent"; percent: Percent } | { type: "Inch"; inch: Inch } | { type: "Centimeter"; centimeter: Centimeter } | { type: "Millimeter"; millimeter: Millimeter }

export type Millimeter = number

export type MouseButton = "Left" | "Middle" | "Right" | "Unkown"

export type Node = ({ type: "Frame" } & FrameNode) | ({ type: "Group" } & GroupNode) | ({ type: "Rectangle" } & RectangleNode)

export type Opacity = Percent

/**
 * Represents a paint style.
 */
export type Paint = ({ type: "Solid" } & SolidPaint) | { type: "Image" } | { type: "Gradient" }

export type Percent = number

export type Pixel = number

export type RectangleNode = { translation: Vec2; angleInRadians: number; size: Size; cornerRadii: CornerRadii; visibility: Visibility; fill: Fill[]; blendMode: BlendMode; opacity: Opacity; stroke: Stroke[] }

export type SelectionChangeOutputEvent = { selected: Entity[] }

export type Size = Vec2

export type SolidPaint = { color: Color }

export type SpectaExport = { comp_dtif: CompDtif; svg_comp_input_event: SvgCompInputEvent; svg_comp_output_event: SvgCompOutputEvent }

export type Stroke = { fill: Fill; width: number }

export type SvgAttribute = { type: "Id"; id: SvgElementId } | { type: "Name"; name: string } | { type: "Width"; width: number; unit: SvgMeasurementUnit } | { type: "Height"; height: number; unit: SvgMeasurementUnit } | { type: "Opacity"; opacity: number } | { type: "Transform"; transform: SvgTransformAttribute } | { type: "PatternTransform"; patternTransform: SvgTransformAttribute } | { type: "D"; d: string } | { type: "ClipPath"; clipPath: SvgElementId } | { type: "Fill"; fill: string } | { type: "ReferencedFill"; id: SvgElementId } | { type: "PatternUnits"; patternUnits: SvgUnitsVariant } | { type: "GradientUnits"; gradientUnits: SvgUnitsVariant }

/**
 * Emitted when an attribute of a SvgElement is removed.
 */
export type SvgAttributeRemovedChange = { key: string }

/**
 * Emitted when an attribute of an SvgElement is updated.
 */
export type SvgAttributeUpdatedChange = { key: string; newValue: string }

export type SvgBlendMode = "Normal" | "Multiply" | "Screen" | "Overlay" | "Darken" | "Lighten" | "ColorDodge" | "ColorBurn" | "HardLight" | "SoftLight" | "Difference" | "Exclusion" | "Hue" | "Saturation" | "Color" | "Luminosity"

export type SvgBuilderOutputEvent = ({ type: "ElementChanges" } & SvgElementChangesOutputEvent)

export type SvgCompInputEvent = { type: "Comp"; event: CompCoreInputEvent } | { type: "Interaction"; event: InteractionInputEvent }

export type SvgCompOutputEvent = ({ type: "ElementChanges" } & SvgElementChangesOutputEvent) | ({ type: "WatchedEntityChanges" } & WatchedEntityChangesOutputEvent) | ({ type: "SelectionChange" } & SelectionChangeOutputEvent)

export type SvgDisplayStyle = "Block" | "None"

/**
 * Emitted when a SvgElement (child) is append to another SvgElement (parent).
 */
export type SvgElementAppendedChange = { parentId: SvgElementId }

export type SvgElementChange = ({ type: "ElementCreated" } & SvgElementCreatedChange) | ({ type: "ElementDeleted" }) | ({ type: "ElementAppended" } & SvgElementAppendedChange) | ({ type: "AttributeUpdated" } & SvgAttributeUpdatedChange) | ({ type: "AttributeRemoved" } & SvgAttributeRemovedChange) | ({ type: "StyleUpdated" } & SvgStyleUpdatedChange) | ({ type: "StyleRemoved" } & SvgStyleRemovedChange)

export type SvgElementChanges = { id: SvgElementId; changes: SvgElementChange[] }

export type SvgElementChangesOutputEvent = { changes: SvgElementChanges }

/**
 * Emitted when a new SvgElement is created.
 */
export type SvgElementCreatedChange = { tagName: string; attributes: ([string, string])[]; styles: ([string, string])[]; parentId: SvgElementId | null; entity: Entity | null }

/**
 * Emitted when a new SvgElement is deleted.
 */
export type SvgElementDeletedChange = Record<string, never>

export type SvgElementId = number

export type SvgMeasurementUnit = "Pixel" | "Percent"

export type SvgStyle = { type: "Display"; display: SvgDisplayStyle } | { type: "BlendMode"; blendMode: SvgBlendMode }

/**
 * Emitted when a style property of a SvgElement is removed.
 */
export type SvgStyleRemovedChange = { key: string }

/**
 * Emitted when a style property of a SvgElement is updated.
 */
export type SvgStyleUpdatedChange = { key: string; newValue: string }

export type SvgTransformAttribute = { type: "Matrix"; a: number; b: number; c: number; d: number; tx: number; ty: number } | { type: "Rotate"; rotation: number }

export type SvgUnitsVariant = "UserSpaceOnUse" | "ObjectBoundingBox"

/**
 * A styled text segment.
 */
export type TextSpan = { 
/**
 * Text content.
 */
text: string; 
/**
 * Font metadata.
 */
font: FontMetadata; 
/**
 * Style properties.
 */
style: TextStyle }

/**
 * Style properties for a text segment.
 */
export type TextStyle = { 
/**
 * Glyph height in pixels, may scale with window.
 */
fontSize: number; 
/**
 * Character spacing.
 */
letterSpacing: LetterSpacing; 
/**
 * Line spacing.
 */
lineHeight: LineHeight }

export type Vec2 = [number, number]

export type Vec4 = [number, number, number, number]

export type VerticalTextAlignment = "Top" | "Center" | "Bottom" | "Justified"

export type Viewport = { physicalPosition: Vec2; physicalSize: Vec2 }

export type Visibility = "Visible" | "Hidden"

export type WatchableMixinVariant = "Dimension" | "Transform"

export type WatchedEntityChangesOutputEvent = { entity: Entity; changes: null[] }

export type WheeledOnCompInputEvent = { position: Vec2; delta: Vec2; ctrlKeyPressed: boolean; metaKeyPressed: boolean }

export type XYWH = { position: Vec2; width: number; height: number }
