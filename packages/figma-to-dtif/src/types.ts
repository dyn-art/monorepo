export type TExportConfig = { mode: 'Inline' } | ({ mode: 'External' } & TExportExternalConfig);

export type TExportImageConfig = { format: 'PNG' | 'JPG' } & TExportConfig;

export interface TExportExternalConfig {
	uploadData: TUploadStaticData;
}

export interface TContentType {
	mimeType: 'image/jpeg' | 'image/png' | 'image/svg+xml' | 'image/gif' | string;
	// ending: `.${string}`;
}

export type TUploadStaticData = (
	content: Uint8Array,
	config?: {
		contentType?: TContentType;
		key?: string;
	}
) => Promise<TUploadStaticDataResponse>;

export interface TUploadStaticDataResponse {
	url: string;
}

export type TFigmaNodeWithChildren = FrameNode | InstanceNode | ComponentNode | GroupNode;
export type TFigmaShapeNode = RectangleNode | EllipseNode | PolygonNode | StarNode | VectorNode;
export type TFigmaNodeWithFills =
	| FrameNode
	| InstanceNode
	| ComponentNode
	| TFigmaShapeNode
	| TextNode
	| VectorNode;
export type TFigmaNodeWithStrokes =
	| FrameNode
	| InstanceNode
	| ComponentNode
	| TFigmaShapeNode
	| TextNode
	| VectorNode;
export type TFigmaNodeWithEffects =
	| FrameNode
	| InstanceNode
	| ComponentNode
	| TFigmaShapeNode
	| TextNode
	| VectorNode;
export type TFigmaLayoutNode =
	| TFigmaShapeNode
	| TextNode
	| FrameNode
	| InstanceNode
	| ComponentNode;

export type TFigmaFormat = 'JPG' | 'PNG' | 'SVG' | 'PDF';
