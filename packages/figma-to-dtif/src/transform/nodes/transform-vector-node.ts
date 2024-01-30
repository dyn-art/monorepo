import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformVectorNode(
	node: VectorNode,
	config: TTransformVectorNodeConfig
): { type: 'Vector' } & COMP.VectorNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Vector',
		node: {
			node_type: 'Vector'
		},
		name: node.name,
		vertices: parseSVGPaths(node.vectorPaths as VectorPath[]),
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
		dimension: {
			height: node.height,
			width: node.width
		},
		relativeTransform: mapFigmaTransformToMat3(node.relativeTransform),
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(node.blendMode),
			opacity: node.opacity,
			isMask: node.isMask
		},
		fill: { paintIds }
	};
}

function parseSVGPaths(vectorPaths: VectorPath[]): COMP.Anchor[] {
	return vectorPaths
		.map((vectorPath) => parseSVGPath(vectorPath.data))
		.reduce((acc, anchors) => acc.concat(anchors), []);
}

// https://www.figma.com/plugin-docs/api/properties/VectorPath-data/
function parseSVGPath(svgPath: string): COMP.Anchor[] {
	const commands = svgPath.match(/[MmLlHhVvCcSsQqTtAaZz][^MmLlHhVvCcSsQqTtAaZz]*/g);
	if (commands == null) {
		return [];
	}

	const anchors: COMP.Anchor[] = [];
	const currentPosition: COMP.Vec2 = [0, 0];

	for (const command of commands) {
		const type = command.charAt(0);
		const args = parseArgs(command.slice(1));

		switch (type) {
			case 'M':
			case 'L':
				processLineCommand(type, args, anchors);
				break;
			case 'C':
				processCurveCommand(args, anchors);
				break;
			case 'Z':
				processCloseCommand(currentPosition, anchors);
				break;
			default:
			// do nothing
		}
	}

	return anchors;
}

function parseArgs(argsStr: string): number[] {
	return argsStr
		.trim()
		.split(/[\s,]+/)
		.map(Number);
}

function processLineCommand(type: string, args: number[], anchors: COMP.Anchor[]): void {
	const currentPosition: COMP.Vec2 = [args[0], args[1]] as unknown as COMP.Vec2;
	anchors.push({
		position: currentPosition,
		command: { type: type === 'M' ? 'MoveTo' : 'LineTo' }
	});
}

function processCurveCommand(args: number[], anchors: COMP.Anchor[]): void {
	const controlPoint1: COMP.Vec2 = [args[0], args[1]] as unknown as COMP.Vec2;
	const controlPoint2: COMP.Vec2 = [args[2], args[3]] as unknown as COMP.Vec2;
	const currentPosition: COMP.Vec2 = [args[4], args[5]] as unknown as COMP.Vec2;
	anchors.push({
		position: currentPosition,
		command: { type: 'CurveTo', controlPoint1, controlPoint2 }
	});
}

function processCloseCommand(currentPosition: COMP.Vec2, anchors: COMP.Anchor[]): void {
	anchors.push({
		position: currentPosition,
		command: { type: 'ClosePath' }
	});
}

interface TTransformVectorNodeConfig {
	paintIds: number[];
}
