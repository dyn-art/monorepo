export type TExportConfig = { mode: 'Inline' } | ({ mode: 'External' } & TExternalConfig);

export type TExportImageConfig = { format: 'PNG' | 'JPG' } & TExportConfig;

export interface TExternalConfig {
	uploadData: TUploadStaticData;
}

export interface TContentType {
	mimeType: 'image/jpeg' | 'image/png' | 'image/svg+xml' | 'image/gif' | string;
	// ending: `.${string}`;
}

export type TUploadStaticData = (
	content: Uint8Array,
	contentType?: TContentType
) => Promise<TUploadStaticDataResponse>;

export interface TUploadStaticDataResponse {
	url: string;
}

export type TFigmaNodeWithChildren = FrameNode | InstanceNode | ComponentNode | GroupNode;
export type TFigmaShapeNode = RectangleNode | EllipseNode | PolygonNode | StarNode;
export type TFigmaNodeWithPaints =
	| FrameNode
	| InstanceNode
	| ComponentNode
	| TFigmaShapeNode
	| TextNode;
