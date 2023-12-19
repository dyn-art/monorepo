export type TExportOptions = { inline: true } | { upload: TUploadStaticData };

export interface TContentType {
	mimeType: 'image/jpeg' | 'image/png' | 'image/svg+xml' | 'image/gif' | string;
	// ending: `.${string}`;
}

export type TUploadStaticData = (
	content: Uint8Array,
	contentType?: TContentType
) => Promise<TUploadStaticDataResponse>;

export interface TUploadStaticDataResponse {
	key: string;
	url?: string;
}

export type TFigmaNodeWithChildren = FrameNode | InstanceNode | ComponentNode | GroupNode;
export type TFigmaShapeNode = RectangleNode | EllipseNode | PolygonNode | StarNode;
export type TFigmaNodeWithPaints =
	| FrameNode
	| InstanceNode
	| ComponentNode
	| TFigmaShapeNode
	| TextNode;
