import type {
	BlendMixin,
	CompositionMixin,
	Entity,
	LayoutMixin,
	PathMixin,
	RenderChange,
	ToJsEvent
} from '@rust/dyn-dtom/bindings';
import { notEmpty, type Unarray } from '@dyn/utils';

import {
	enqueueJsEvents,
	transformRustEnumArrayToObject,
	type GroupedRustEnums
} from '../../../wasm';
import type { Composition } from '../../composition';
import { Renderer } from '../Renderer';
import { createSVGNode, type SVGNode } from './SVGNode';

export class SVGRenderer extends Renderer {
	private _svgElement: SVGNode;
	private _defsElement: SVGNode;

	private _svgNodeMap = new Map<Entity, SVGNode>();

	private _toProcessRenderUpdates: Record<Entity, TToProcessRenderUpdate> = {};
	private _renderedInRenderCycle = new Set<Entity>();

	constructor(composition: Composition, options: TSVGRendererOptions = {}) {
		super(composition);
		const { domElement = document.body } = options;
		this._svgElement = createSVGNode('svg');
		domElement.appendChild(this._svgElement.element);
		this._defsElement = createSVGNode('defs');
		this._svgElement.appendChild(this._defsElement);
		this._svgElement.setStyles({ overflow: 'hidden' });
	}

	public setSize(width: number, height: number): this {
		this._svgElement.setAttributes({ width: `${width}px`, height: `${height}px` });
		// TODO: trigger resize event
		return this;
	}

	public getNodeByEntity(entity: number): SVGNode | null {
		return this._svgNodeMap.get(entity) ?? null;
	}

	// =========================================================================
	// Rendering
	// =========================================================================

	public render(events: ToJsEvent['RenderUpdate'][]): this {
		for (const renderUpdate of events) {
			const groupedChanges: GroupedRustEnums<RenderChange> = transformRustEnumArrayToObject(
				renderUpdate.changes
			);
			const parentMixin =
				groupedChanges.ParentMixin != null && groupedChanges.ParentMixin.length > 0
					? groupedChanges.ParentMixin[0]
					: null;
			const parentId = parentMixin?.parent ?? null;
			this._toProcessRenderUpdates[renderUpdate.entity] = {
				node_type: renderUpdate.node_type,
				changes: groupedChanges,
				parentId
			};
		}

		// Process each render update
		for (const entity of Object.keys(this._toProcessRenderUpdates)) {
			const update = this._toProcessRenderUpdates[Number(entity)];
			if (update != null) {
				this.handleRenderUpdate(Number(entity), update);
			}
		}

		this._toProcessRenderUpdates = {};
		this._renderedInRenderCycle = new Set<number>();

		return this;
	}

	private handleRenderUpdate(entity: Entity, renderUpdate: TToProcessRenderUpdate): this {
		const { node_type: nodeType, parentId } = renderUpdate;

		// If parent exists and hasn't been rendered yet, try to render it first
		if (parentId !== null && !this._renderedInRenderCycle.has(parentId)) {
			const parentUpdate = this._toProcessRenderUpdates[parentId];
			if (parentUpdate != null) {
				this.handleRenderUpdate(parentId, parentUpdate);
			}
		}

		// Render changes
		switch (nodeType) {
			case 'Rectangle':
				this.renderShape(entity, renderUpdate);
				break;
			case 'Group':
			case 'Frame':
				this.renderGroup(entity, renderUpdate);
				break;
			default:
				break;
		}

		return this;
	}

	private renderGroup(enitity: number, renderUpdate: TToProcessRenderUpdate): void {
		const { changes, parentId } = renderUpdate;
		let renderElement = this.getNodeByEntity(enitity);

		// Append SVG element if it doesn't exist yet
		if (renderElement == null) {
			// TODO: Create more advanced Node similar to the plain Typescript POC
			renderElement = createSVGNode('g');
			if (parentId != null) {
				const parentElement = this.getNodeByEntity(parentId);
				if (parentElement != null) {
					parentElement.appendChild(renderElement);
				}
			} else {
				this._svgElement.appendChild(renderElement);
			}
			this._svgNodeMap.set(enitity, renderElement);
			renderElement.setAttributes({ id: `group-${enitity}` });
			renderElement.setStyles({ fill: 'blue' });
			renderElement.onPointerDown(() => {
				console.log('PointerDownEventOnEntity', { enitity });
				enqueueJsEvents(this.composition.worldIds.mainWorldId, [
					{ PointerDownEventOnEntity: { entity: enitity } }
				]);
			});
		}

		if (!renderElement.isVisible) {
			return;
		}

		// Handle Blend change
		const blendChange = this.getLatestChange(changes, 'Blend');
		if (blendChange != null) {
			this.handleBlendChange(renderElement, blendChange);
		}

		// Handle Composition change
		const compositionChange = this.getLatestChange(changes, 'Composition');
		if (compositionChange != null) {
			this.handleCompositionChange(renderElement, compositionChange);
		}
	}

	private renderShape(enitity: number, renderUpdate: TToProcessRenderUpdate): void {
		const { changes, parentId } = renderUpdate;
		let renderElement = this.getNodeByEntity(enitity);

		// Append SVG element if it doesn't exist yet
		if (renderElement == null) {
			// TODO: Create more advanced Node similar to the plain Typescript POC
			renderElement = createSVGNode('path');
			if (parentId != null) {
				const parentElement = this.getNodeByEntity(parentId);
				if (parentElement != null) {
					parentElement.appendChild(renderElement);
				}
			} else {
				this._svgElement.appendChild(renderElement);
			}
			this._svgNodeMap.set(enitity, renderElement);
			renderElement.setAttributes({ id: `shape-${enitity}` });
			renderElement.setStyles({ fill: 'red' });
			renderElement.onPointerDown(() => {
				console.log('PointerDownEventOnEntity', { enitity });
				enqueueJsEvents(this.composition.worldIds.mainWorldId, [
					{ PointerDownEventOnEntity: { entity: enitity } }
				]);
			});
		}

		if (!renderElement.isVisible) {
			return;
		}

		// Handle Path change
		const pathChange = this.getLatestChange(changes, 'Path');
		if (pathChange != null) {
			this.handlePathChange(renderElement, pathChange);
		}

		// Handle Layout change
		const layoutChange = this.getLatestChange(changes, 'Layout');
		if (layoutChange != null) {
			this.handleLayoutChange(renderElement, layoutChange);
		}

		// Handle Blend change
		const blendChange = this.getLatestChange(changes, 'Blend');
		if (blendChange != null) {
			this.handleBlendChange(renderElement, blendChange);
		}

		// Handle Composition change
		const compositionChange = this.getLatestChange(changes, 'Composition');
		if (compositionChange != null) {
			this.handleCompositionChange(renderElement, compositionChange);
		}

		// TODO:
	}

	private renderFill(enitityId: number, changes: RenderChange[]): void {
		// TODO
	}

	private renderText(enitityId: number, changes: RenderChange[]): void {
		// TODO
	}

	// =========================================================================
	// Render Update Handler
	// =========================================================================

	private handlePathChange(renderElement: SVGNode, mixin: PathMixin): void {
		const svgPath = this.constructSVGPath(mixin);
		renderElement.setAttributes({ d: svgPath });
	}

	private handleLayoutChange(renderElement: SVGNode, mixin: LayoutMixin): void {
		const [aX, aY, , bX, bY, , cX, cY] = mixin.relative_transform;
		renderElement.setAttributes({
			transform: `matrix(${aX}, ${aY}, ${bX}, ${bY}, ${cX}, ${cY})`,
			width: mixin.width,
			height: mixin.height
		});
	}

	private handleBlendChange(renderElement: SVGNode, mixin: BlendMixin): void {
		renderElement.setAttributes({ opacity: mixin.opacity });
	}

	private handleCompositionChange(renderElement: SVGNode, mixin: CompositionMixin): void {
		renderElement.isVisible = mixin.is_visible;
		renderElement.setStyles({ display: renderElement.isVisible ? 'block' : 'none' });
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
				const [x, y] = anchor.position;
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
			.filter(notEmpty);

		return pathCommands.join(' ');
	}

	// =========================================================================
	// Helper
	// =========================================================================

	private getLatestChange<
		TChanges extends Record<string, unknown[]> = GroupedRustEnums<RenderChange>,
		TKey extends keyof TChanges = keyof TChanges
	>(changes: TChanges, elementType: TKey): Unarray<TChanges[TKey]> | null {
		const change = changes[elementType];
		if (change != null && change.length > 0) {
			return change[change.length - 1] as Unarray<TChanges[TKey]>;
		}
		return null;
	}
}

export interface TSVGRendererOptions {
	domElement?: Element;
}

type TToProcessRenderUpdate = {
	changes: GroupedRustEnums<RenderChange>;
	parentId: Entity | null;
} & Omit<Omit<ToJsEvent['RenderUpdate'], 'changes'>, 'entity'>;
