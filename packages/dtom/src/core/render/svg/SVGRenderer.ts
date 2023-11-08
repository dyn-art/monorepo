import { notEmpty, type Unarray } from '@dyn/utils';
import type {
	BlendMixin,
	DimensionMixin,
	Entity,
	NodeCompositionMixin,
	PathMixin,
	RenderChange,
	RenderChangeRelativeTransformMixin,
	RenderUpdateEvent
} from '@/rust/dyn_composition_api/bindings';

import type { Composition } from '../../composition';
import { groupByType, transformToCSS, type GroupedByType } from '../../helper';
import { Renderer } from '../Renderer';
import { createSVGNode, type SVGNode } from './SVGNode';

// TODO: Refactor and create established SVG structure (see old POC)
export class SVGRenderer extends Renderer {
	private _svgElement: SVGNode;
	private _defsElement: SVGNode;
	private _domElement: Element;

	private _svgNodeMap = new Map<Entity, SVGNode>();

	private _toProcessRenderUpdates: Record<Entity, TToProcessRenderUpdate> = {};
	private _renderedInRenderCycle = new Set<Entity>();

	constructor(composition: Composition, options: TSVGRendererOptions = {}) {
		super(composition);
		const { domElement = document.body } = options;
		this._domElement = domElement;
		this._svgElement = createSVGNode('svg');
		this._domElement.appendChild(this._svgElement.element);
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

	public destroy(): this {
		while (this._domElement.firstChild) {
			this._domElement.removeChild(this._domElement.firstChild);
		}
		return this;
	}

	// =========================================================================
	// Rendering
	// =========================================================================

	public render(events: RenderUpdateEvent[]): this {
		for (const renderUpdate of events) {
			const groupedChanges = groupByType(renderUpdate.changes);
			const parentMixin =
				groupedChanges.ParentMixin != null && groupedChanges.ParentMixin.length > 0
					? groupedChanges.ParentMixin[0]
					: null;
			const parentId = parentMixin?.parent ?? null;
			this._toProcessRenderUpdates[renderUpdate.entity] = {
				nodeType: renderUpdate.nodeType,
				changes: groupedChanges,
				parentId
			};
		}

		// TODO: Make it with one update cycle work
		// e.g. relative_transform on creation is not applied
		// TODO: Not getting here and parent mixin missing in children
		// console.log('after render presort', { toProcessRenderUpdates: this._toProcessRenderUpdates }); // TODO: REMOVE

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
		const { nodeType, parentId } = renderUpdate;

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
				this.composition.emitEvents([{ type: 'CursorDownOnEntity', entity: enitity }]);
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
				this.composition.emitEvents([{ type: 'CursorDownOnEntity', entity: enitity }]);
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

		// Handle Relative Transform change
		const relativeTransformChange = this.getLatestChange(changes, 'RelativeTransform');
		if (relativeTransformChange != null) {
			this.handleRelativeTransformChange(renderElement, relativeTransformChange);
		}

		// Handle Dimension change
		const dimensionChange = this.getLatestChange(changes, 'Dimension');
		if (dimensionChange != null) {
			this.handleDimensionChange(renderElement, dimensionChange);
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

	private handleRelativeTransformChange(
		renderElement: SVGNode,
		mixin: RenderChangeRelativeTransformMixin
	): void {
		// Note: Setting transform attribute is like 30% faster
		renderElement.setAttributes({
			transform: transformToCSS(mixin.relativeTransform, true).transform
		});
	}

	private handleDimensionChange(renderElement: SVGNode, mixin: DimensionMixin): void {
		renderElement.setAttributes({
			width: mixin.width,
			height: mixin.height
		});
	}

	private handleBlendChange(renderElement: SVGNode, mixin: BlendMixin): void {
		renderElement.setAttributes({ opacity: mixin.opacity });
	}

	private handleCompositionChange(renderElement: SVGNode, mixin: NodeCompositionMixin): void {
		renderElement.isVisible = mixin.isVisible;
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
						return `A ${rx} ${ry} ${arcParams.xAxisRotation} ${boolToNum(
							arcParams.largeArcFlag
						)} ${boolToNum(arcParams.sweepFlag)} ${x} ${y}`;
					} else if ('CurveTo' in anchorCommand) {
						const curveParams = anchorCommand.CurveTo;
						return `C ${curveParams.controlPoint1[0]} ${curveParams.controlPoint1[1]} ${curveParams.controlPoint2[0]} ${curveParams.controlPoint2[1]} ${x} ${y}`;
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
		TChanges extends Record<string, unknown[]> = GroupedByType<RenderChange>,
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
	changes: GroupedByType<RenderChange>;
	parentId: Entity | null;
} & Omit<Omit<RenderUpdateEvent, 'changes'>, 'entity'>;
