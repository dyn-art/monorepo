export type TExportOptions = { inline: boolean } | { upload: TUploadStaticData };

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
