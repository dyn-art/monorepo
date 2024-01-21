import { ExportImagePaintException } from '../../exceptions';

export async function exportFigmaImageData(
	imageHash: string,
	nodeIds: SceneNode['id'][]
): Promise<TExportFigmaImageDataResponse> {
	let content: TExportFigmaImageDataResponse['content'] | null = null;
	let size: TExportFigmaImageDataResponse['size'] | null = null;

	try {
		const image = figma.getImageByHash(imageHash);
		if (image != null) {
			content = await image.getBytesAsync();
			size = await image.getSizeAsync();
		}
	} catch (error) {
		throw new ExportImagePaintException(nodeIds, error);
	}

	if (content == null || size == null) {
		throw new ExportImagePaintException(nodeIds);
	}

	return { content, size };
}

export interface TExportFigmaImageDataResponse {
	content: Uint8Array;
	size: { width: number; height: number };
}
