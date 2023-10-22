import { notEmpty } from '@dyn/utils';

import { Renderer, type TRendererOptions } from '../Renderer';
import type { SVGNode } from './SVGNode';

export class SVGRenderer extends Renderer {
	private _domElement: SVGElement;
	private _defsElement: SVGElement;

	private readonly _version = '1.1';
	private readonly _ns = 'http://www.w3.org/2000/svg';
	private readonly _xlink = 'http://www.w3.org/1999/xlink';

	private _entityMap = new Map<number, SVGNode>();

	constructor(options: TSVGRendererOptions = {}) {
		super(options);
		const { domElement = this.createSVGElement('svg') } = options;
		this._domElement = domElement;
		this._defsElement = this.createSVGElement('defs');
		this._domElement.appendChild(this._defsElement);
		this._domElement.style.overflow = 'hidden';
	}

	public setSize(width: number, height: number): this {
		this._width = width;
		this._height = height;
		this.setAttributes(this._domElement, { width: `${width}px`, height: `${height}px` });
		// TODO: trigger resize event
		return this;
	}

	public getNodeByEntityId(entityId: number): SVGNode | null {
		return this._entityMap.get(entityId) ?? null;
	}

	public render(data: { entity: number; changes: any[] }): this {
		// TODO:
		console.log('Called SVG render', { data });

		// 1. Check whether element for entity already exists
		// 2. If not, create new element and append to parent or DOM
		// 3. Register callbacks for changes

		for (const change of data.changes) {
			if (change.Path != null) {
				const pathString = this.constructPathString(change.Path as Path);
				console.log({ pathString });
			}
		}

		return this;
	}

	private constructPathString(path: Path): string {
		// Helper function to translate boolean flags to 1 or 0 for SVG
		const boolToNum = (flag: boolean) => (flag ? '1' : '0');

		// Map path vertices to SVG path commands
		// and join them into a string
		return path.vertices
			.map((anchor) => {
				const [x, y] = anchor.position;
				const anchorCommand = anchor.command;

				if (typeof anchorCommand === 'string') {
					switch (anchor.command) {
						case 'MoveTo':
							return `M ${x} ${y}`;
						case 'LineTo':
							return `L ${x} ${y}`;
						case 'ClosePath':
							return 'Z';
						default:
							return null;
					}
				} else if (typeof anchorCommand === 'object') {
					if ('ArcTo' in anchorCommand) {
						const arcParams = anchorCommand.ArcTo;
						const [rx, ry] = arcParams.radius;
						return `A ${rx} ${ry} ${arcParams.x_axis_rotation} ${boolToNum(
							arcParams.large_arc_flag
						)} ${boolToNum(arcParams.sweep_flag)} ${x} ${y}`;
					} else if ('CurveTo' in anchorCommand) {
						const curveParams = anchorCommand.CurveTo;
						return `C ${curveParams.control_point_1[0]} ${curveParams.control_point_1[1]} ${curveParams.control_point_2[0]} ${curveParams.control_point_2[1]} ${x} ${y}`;
					}
				}

				return null;
			})
			.filter(notEmpty)
			.join(' ');
	}

	// Create a namespaced SVG element
	private createSVGElement(tag: TSVGTagNames, attributes: Record<string, string> = {}): SVGElement {
		const element = document.createElementNS(this._ns, tag);
		if (tag === 'svg' && attributes.version == null) {
			attributes.version = this._version;
		}
		if (Object.keys(attributes).length > 0) {
			this.setAttributes(element, attributes);
		}
		return element;
	}

	// Add attributes to SVG element
	private setAttributes(element: SVGElement, attributes: Record<string, string>): this {
		for (const [key, value] of Object.entries(attributes)) {
			if (key.includes('href')) {
				element.setAttributeNS(this._xlink, key, value);
			} else {
				element.setAttribute(key, value);
			}
		}
		return this;
	}

	// Remove attributes from SVG element
	private removeAttributes(element: Element, attributeKeys: string[]): this {
		for (const attr of attributeKeys) {
			element.removeAttribute(attr);
		}
		return this;
	}
}

export type TSVGRendererOptions = {
	domElement?: SVGElement;
} & TRendererOptions;

export type TSVGTagNames =
	| 'svg'
	| 'circle'
	| 'ellipse'
	| 'line'
	| 'path'
	| 'polygon'
	| 'polyline'
	| 'rect'
	| 'text'
	| 'use'
	| 'defs'
	| 'g'
	| 'symbol';

// TODO: REMOVE - Only temp until specta changes merged for type generation

type Vec2 = [number, number];

interface ArcTo {
	radius: Vec2;
	x_axis_rotation: number;
	large_arc_flag: boolean;
	sweep_flag: boolean;
}

interface CurveTo {
	control_point_1: Vec2;
	control_point_2: Vec2;
}

type Command = 'MoveTo' | 'LineTo' | { CurveTo: CurveTo } | 'ClosePath' | { ArcTo: ArcTo };

interface Anchor {
	position: Vec2;
	command: Command;
}

interface Path {
	vertices: Anchor[];
}
