import { ExportNodeException } from '../../exceptions';

export async function exportFigmaNode(
	node: SceneNode,
	settings: ExportSettings
): Promise<Uint8Array> {
	const { format = 'PNG', ...rest } = settings;
	try {
		return await node.exportAsync({
			format,
			...rest
		});
	} catch (error) {
		throw new ExportNodeException(format, node);
	}
}
