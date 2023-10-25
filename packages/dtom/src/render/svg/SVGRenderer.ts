import type { PathMixin, RenderChange, ToJsEvent } from '@rust/dyn-dtom/bindings';
import { notEmpty } from '@dyn/utils';

import type { Composition } from '../../composition';
import { Renderer } from '../Renderer';
import { SVGNode } from './SVGNode';

export class SVGRenderer extends Renderer {
	private _domElement: SVGElement;
	private _defsElement: SVGElement;

	private readonly _version = '1.1';
	private readonly _ns = 'http://www.w3.org/2000/svg';
	private readonly _xlink = 'http://www.w3.org/1999/xlink';

	private _entityMap = new Map<number, SVGNode>();

	constructor(composition: Composition, options: TSVGRendererOptions = {}) {
		super(composition);
		const { domElement = this.createSVGElement('svg') } = options;
		this._domElement = domElement;
		this._defsElement = this.createSVGElement('defs');
		this._domElement.appendChild(this._defsElement);
		this._domElement.style.overflow = 'hidden';
	}

	public setSize(width: number, height: number): this {
		this.setAttributes(this._domElement, { width: `${width}px`, height: `${height}px` });
		// TODO: trigger resize event
		return this;
	}

	public getNodeByEntityId(entityId: number): SVGNode | null {
		return this._entityMap.get(entityId) ?? null;
	}

	// TODO
	public getOrCreateNodeByEntityId(entityId: number, parentEntityId?: number): SVGNode {
		let node = this._entityMap.get(entityId);
		if (node == null) {
			const parentSVGElement =
				(parentEntityId != null ? this.getNodeByEntityId(parentEntityId)?.element : null) ??
				this._domElement;
			parentSVGElement.appendChild(this.createSVGElement('path'));
			node = new SVGNode(parentSVGElement);
			this._entityMap.set(entityId, node);
		}
		return node;
	}

	public render(data: ToJsEvent['RenderUpdate']): this {
		const { changes, entity: entityId, node_type: nodeType } = data;

		// TODO:
		console.log('Called SVG render', { changes, entityId, nodeType });

		// 1. Check whether element for entity already exists
		// 2. If not, create new element and append to parent or DOM
		// 3. Register callbacks for changes

		switch (nodeType) {
			case 'Rectangle':
				this.renderPath(entityId, changes);
				break;
			case 'Group':
			case 'Frame':
				this.renderGroup(entityId, changes);
				break;
			default:
				break;
		}

		return this;
	}

	public renderGroup(enitityId: number, changes: RenderChange[]): this {
		return this;
	}

	public renderPath(enitityId: number, changes: RenderChange[]): this {
		for (const change of changes) {
			if ('Path' in change) {
				const pathParams = change.Path;
				const svgPath = this.constructSVGPath(pathParams);
				console.log({ pathString: svgPath });
			}
		}
		return this;
	}

	public renderFill(enitityId: number, changes: RenderChange[]): this {
		return this;
	}

	public renderText(enitityId: number, changes: RenderChange[]): this {
		return this;
	}

	// =========================================================================
	// SVG
	// =========================================================================

	private constructSVGPath(pathParams: PathMixin): string {
		// Helper function to translate boolean flags to 1 or 0 for SVG
		const boolToNum = (flag: boolean): string => (flag ? '1' : '0');

		// Map path vertices to SVG path commands
		const pathCommands: string[] = pathParams.vertices
			.map((anchor) => {
				const { x, y } = anchor.position;
				const anchorCommand = anchor.command;

				// Handle anchor commands without parameters
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
				}

				// Handle anchor commands with parameters
				else if (typeof anchorCommand === 'object') {
					if ('ArcTo' in anchorCommand) {
						const arcParams = anchorCommand.ArcTo;
						const { x: rx, y: ry } = arcParams.radius;
						return `A ${rx} ${ry} ${arcParams.x_axis_rotation} ${boolToNum(
							arcParams.large_arc_flag
						)} ${boolToNum(arcParams.sweep_flag)} ${x} ${y}`;
					} else if ('CurveTo' in anchorCommand) {
						const curveParams = anchorCommand.CurveTo;
						return `C ${curveParams.control_point_1.x} ${curveParams.control_point_1.y} ${curveParams.control_point_2.x} ${curveParams.control_point_2.y} ${x} ${y}`;
					}
				}

				return null;
			})
			.filter(notEmpty);

		return pathCommands.join(' ');
	}

	// =========================================================================
	// DOM
	// =========================================================================

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

export interface TSVGRendererOptions {
	domElement?: SVGElement;
}

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
