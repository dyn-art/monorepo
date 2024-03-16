import { intoMouseButton } from '@dyn/dtif-comp';
import type {
	CompositionChangeOutputEvent,
	SvgElementChangesOutputEvent,
	SvgElementId,
	Vec2
} from '@/rust/dyn-svg-comp-api/bindings';

import type { Composition } from '../Composition';
import { Renderer } from './Renderer';

export const VERSION = '1.1';
export const NS = 'http://www.w3.org/2000/svg';
export const XLINK = 'http://www.w3.org/1999/xlink';

export class SvgRenderer extends Renderer {
	private _domElement: Element;
	private _svgElement: SVGElement;

	private _svgElementMap = new Map<SvgElementId, SVGElement>();

	private _cursorInCompBounds = false;

	constructor(composition: Composition, options: TsvgRendererOptions = {}) {
		super(composition, options.callbackBased ?? true);
		const { domElement = document.body } = options;
		this._domElement = domElement;

		// Create SVG root
		this._svgElement = document.createElementNS(NS, 'svg');
		this._svgElement.setAttribute('version', VERSION);
		this._svgElement.setAttribute('id', 'svg-canvas');
		this._svgElement.style.setProperty('overflow', 'hidden');
		// this._svgElement.style.setProperty('pointer-events', 'none');
		this._domElement.appendChild(this._svgElement);

		// Register SVG root callbacks
		this._svgElement.addEventListener('pointermove', (e) => {
			e.preventDefault();
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'CursorMovedOnComposition',
						position: this.pointerEventToCompPoint(e)
					}
				},
				false
			);
		});
		this._svgElement.addEventListener('wheel', (e) => {
			e.preventDefault();
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'WheeledOnComposition',
						position: this.clientWindowPointToCompPoint([e.clientX, e.clientY]),
						ctrlKeyPressed: e.ctrlKey,
						metaKeyPressed: e.metaKey,
						delta: [e.deltaX, e.deltaY]
					}
				},
				false
			);
		});
		this._svgElement.addEventListener('pointerdown', (e) => {
			e.preventDefault();
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'CursorDownOnComposition',
						position: this.pointerEventToCompPoint(e),
						button: intoMouseButton(e.button)
					}
				},
				true
			);
		});
		this._svgElement.addEventListener('pointerup', (e) => {
			e.preventDefault();
			this.composition.emitInputEvent(
				{
					type: 'Interaction',
					event: {
						type: 'CursorUpOnComposition',
						position: this.pointerEventToCompPoint(e),
						button: intoMouseButton(e.button)
					}
				},
				true
			);
		});
		this._svgElement.addEventListener('pointerenter', (e) => {
			e.preventDefault();
			if (!this._cursorInCompBounds) {
				this.composition.emitInputEvent(
					{
						type: 'Interaction',
						event: {
							type: 'CursorEnteredComposition'
						}
					},
					false
				);
				this._cursorInCompBounds = true;
			}
		});
		this._svgElement.addEventListener('pointerleave', (e) => {
			e.preventDefault();
			const compPoint = this.pointerEventToCompPoint(e);
			// Check whether cursor actually left composition
			// or whether its just on some UI layer like the selection box
			if (
				this._cursorInCompBounds &&
				(compPoint[0] < 0 ||
					compPoint[0] > this.composition.size[0] ||
					compPoint[1] < 0 ||
					compPoint[1] > this.composition.size[1])
			) {
				this.composition.emitInputEvent(
					{
						type: 'Interaction',
						event: {
							type: 'CursorExitedComposition'
						}
					},
					false
				);
				this._cursorInCompBounds = false;
			}
		});
	}

	public applyElementChanges(event: SvgElementChangesOutputEvent): void {
		let element: SVGElement | null = null;
		const getElement = (): SVGElement | null => {
			if (element == null) {
				element = this._svgElementMap.get(event.id) ?? null;
			}
			return element;
		};

		for (const change of event.changes) {
			switch (change.type) {
				case 'ElementCreated': {
					const newElement: SVGElement = document.createElementNS(NS, change.tagName);

					// Apply attributes
					for (const [key, value] of change.attributes) {
						newElement.setAttribute(key, value);
					}

					// Apply styles
					for (const [key, value] of change.styles) {
						newElement.style.setProperty(key, value);
					}

					// Register callbacks
					const entity = change.entity;
					if (entity != null) {
						newElement.addEventListener('pointerdown', (e) => {
							e.preventDefault();
							this.composition.emitInputEvent({
								type: 'Interaction',
								event: {
									type: 'CursorDownOnEntity',
									entity,
									position: this.pointerEventToCompPoint(e),
									button: intoMouseButton(e.button)
								}
							});
						});
					}

					// Append element to parent
					if (change.parentId != null) {
						const parentElement = this._svgElementMap.get(change.parentId);
						if (parentElement != null) {
							parentElement.appendChild(newElement);
						} else {
							console.error(
								`Failed to query parent element (${change.parentId}) to append element (${event.id}) to!`
							);
						}
					} else {
						this._svgElement.appendChild(newElement);
					}

					this._svgElementMap.set(event.id, newElement);
					element = newElement;
					break;
				}
				case 'ElementDeleted': {
					const elementToDelete = getElement();
					if (elementToDelete?.parentNode != null) {
						elementToDelete.parentNode.removeChild(elementToDelete);
						this._svgElementMap.delete(event.id);
					} else {
						console.error(`Failed to query to remove element (${event.id})!`);
					}
					break;
				}
				case 'ElementAppended': {
					const toAppendElement = getElement();
					const parentElement = this._svgElementMap.get(change.parentId);
					if (parentElement != null && toAppendElement != null) {
						parentElement.appendChild(toAppendElement);
					} else {
						console.error(
							`Failed to query parent element (${change.parentId}) to append element (${event.id}) to!`
						);
					}
					break;
				}
				case 'AttributeUpdated': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						elementToUpdate.setAttribute(change.key, change.newValue);
					} else {
						console.error(
							`Failed to query element (${event.id}) to add attribute (${change.key}) to!`
						);
					}
					break;
				}
				case 'AttributeRemoved': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						elementToUpdate.removeAttribute(change.key);
					} else {
						console.error(
							`Failed to query element (${event.id}) to remove attribute (${change.key}) from!`
						);
					}
					break;
				}
				case 'StyleUpdated': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						elementToUpdate.style.setProperty(change.key, change.newValue);
					} else {
						console.error(`Failed to query element (${event.id}) to add style (${change.key}) to!`);
					}
					break;
				}
				case 'StyleRemoved': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						elementToUpdate.style.removeProperty(change.key);
					} else {
						console.error(
							`Failed to query element (${event.id}) to remove style (${change.key}) from!`
						);
					}
					break;
				}
				case 'ElementChildrenReordered': {
					const elementToUpdate = getElement();
					if (elementToUpdate != null) {
						// Iterate over the 'order' array and append each child to 'elementToUpdate'
						// in the new order. This effectively reorders the children.
						for (const id of change.newOrder) {
							const childElement = document.getElementById(id.toString());
							if (childElement != null) {
								elementToUpdate.appendChild(childElement);
							} else {
								console.error(`Failed to query child element (${id}) to reorder!`);
							}
						}
					} else {
						console.error(`Failed to query element (${event.id}) to apply new children order!`);
					}
					break;
				}
			}
		}
	}

	public applyCompositionChange(event: CompositionChangeOutputEvent): void {
		this._svgElement.setAttribute('width', `${event.size[0]}px`);
		this._svgElement.setAttribute('height', `${event.size[1]}px`);
		this._svgElement.setAttribute(
			'viewBox',
			`${event.viewport.physicalPosition[0]} ${event.viewport.physicalPosition[1]} ${event.viewport.physicalSize[0]} ${event.viewport.physicalSize[1]}`
		);
	}

	public clear(): void {
		this._svgElementMap.clear();
		while (this._domElement.firstChild) {
			this._domElement.removeChild(this._domElement.firstChild);
		}
	}

	public clientWindowPointToCompPoint(clientPoint: Vec2): Vec2 {
		const rect = this._svgElement.getBoundingClientRect();

		const x = clientPoint[0] - rect.left;
		const y = clientPoint[1] - rect.top;

		return [x, y];
	}
}

export interface TsvgRendererOptions {
	domElement?: Element;
	callbackBased?: boolean;
}
