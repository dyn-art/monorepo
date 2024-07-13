import { shortId } from '@ibg/utils';
import { toKeyCode, toMouseButton } from '@dyn/arb-dtif';
import type {
	ArtboardChangeOutputEvent,
	SvgElementChangesOutputEvent,
	SvgElementId,
	Vec2
} from '@/rust/dyn-arb-svg-builder-api/bindings';

import type { Artboard } from './Artboard';

export const VERSION = '1.1';
export const NS = 'http://www.w3.org/2000/svg';
export const XLINK = 'http://www.w3.org/1999/xlink';

export class SvgBuilder {
	private readonly _isCallbackBased: boolean;
	private readonly _isInteractive: boolean;
	private readonly _arb: () => Artboard; // TODO: Bad practice?

	private _domElement: HTMLElement;
	private _svgElement: SVGElement;
	private _svgElementId: string;

	private _svgElementMap = new Map<SvgElementId, SVGElement>();

	private _cursorInArbBounds = false;

	constructor(artboard: Artboard, options: TSvgBuilderOptions = {}) {
		const { domElement = document.body, callbackBased = true, interactive = false } = options;
		this._arb = () => artboard;
		this._isCallbackBased = callbackBased;
		this._isInteractive = interactive;
		this._domElement = domElement;
		this._svgElementId = `svg-artboard_${shortId()}`;

		// Create SVG root
		this._svgElement = document.createElementNS(NS, 'svg');
		this._svgElement.setAttribute('version', VERSION);
		this._svgElement.setAttribute('id', this._svgElementId);
		this._svgElement.style.setProperty('overflow', 'hidden');
		this._svgElement.style.setProperty('pointer-events', 'none');
		this._domElement.appendChild(this._svgElement);

		artboard.watchOutputEvent('SvgElementChange', (event) => {
			this.applyElementChanges(event);
		});
		artboard.watchOutputEvent('ArtboardChange', (event) => {
			this.applyArtboardChange(event);
		});

		// Register callbacks
		// Note: To prevent blocking artboard events, non-blocking elements
		// like SelectionBox should be direct children of _domElement.
		// We attach event listeners to the parent, not the SVG directly, to allow
		// event propagation from sibling nodes (like the SelectionBox).
		if (this.isInteractive) {
			this._domElement.addEventListener('pointermove', (e) => {
				e.preventDefault();
				this.artboard.emitInputEvent(
					'Interaction',
					{
						type: 'CursorMovedOnArtboard',
						position: this.pointerEventToArbPoint(e)
					},
					false
				);
			});
			this._domElement.addEventListener('wheel', (e) => {
				e.preventDefault();
				this.artboard.emitInputEvent(
					'Interaction',
					{
						type: 'MouseWheeledOnArtboard',
						position: this.clientWindowPointToArbPoint([e.clientX, e.clientY]),
						delta: [e.deltaX, e.deltaY]
					},
					false
				);
			});
			this._domElement.addEventListener('pointerdown', (e) => {
				e.preventDefault();
				this.artboard.emitInputEvent(
					'Interaction',
					{
						type: 'CursorDownOnArtboard',
						position: this.pointerEventToArbPoint(e),
						button: toMouseButton(e.button)
					},
					true
				);
			});
			this._domElement.addEventListener('pointerup', (e) => {
				e.preventDefault();
				this.artboard.emitInputEvent(
					'Interaction',
					{
						type: 'CursorUpOnArtboard',
						position: this.pointerEventToArbPoint(e),
						button: toMouseButton(e.button)
					},
					true
				);
			});
			this._domElement.addEventListener('pointerenter', (e) => {
				e.preventDefault();
				if (!this._cursorInArbBounds) {
					this.artboard.emitInputEvent(
						'Interaction',
						{
							type: 'CursorEnteredArtboard'
						},
						false
					);
					this._cursorInArbBounds = true;
				}
			});
			this._domElement.addEventListener('pointerleave', (e) => {
				e.preventDefault();
				const arbPoint = this.pointerEventToArbPoint(e);
				// Check whether cursor actually left artboard
				// or whether its just on some UI layer like the selection box
				if (
					this._cursorInArbBounds &&
					(arbPoint[0] < 0 ||
						arbPoint[0] > this.artboard.size[0] ||
						arbPoint[1] < 0 ||
						arbPoint[1] > this.artboard.size[1])
				) {
					this.artboard.emitInputEvent(
						'Interaction',
						{
							type: 'CursorExitedArtboard'
						},
						false
					);
					this._cursorInArbBounds = false;
				}
			});
			window.addEventListener('keydown', (e) => {
				if (this._cursorInArbBounds) {
					e.preventDefault();
					this.artboard.emitInputEvent(
						'Interaction',
						{
							type: 'KeyDownOnArtboard',
							keyCode: toKeyCode(e.code)
						},
						true
					);
				}
			});
			window.addEventListener('keyup', (e) => {
				if (this._cursorInArbBounds) {
					e.preventDefault();
					this.artboard.emitInputEvent(
						'Interaction',
						{
							type: 'KeyUpOnArtboard',
							keyCode: toKeyCode(e.code)
						},
						true
					);
				}
			});
		}
	}

	protected get artboard(): Artboard {
		return this._arb();
	}

	public get isCallbackBased(): boolean {
		return this._isCallbackBased;
	}

	public get isInteractive(): boolean {
		return this._isInteractive;
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
					if (this.isInteractive) {
						const entity = change.entity;
						if (entity != null) {
							newElement.addEventListener('pointerdown', (e) => {
								e.preventDefault();
								this.artboard.emitInputEvent('Interaction', {
									type: 'CursorDownOnEntity',
									entity,
									position: this.pointerEventToArbPoint(e),
									button: toMouseButton(e.button)
								});
							});
						}
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

	public applyArtboardChange(event: ArtboardChangeOutputEvent): void {
		this._svgElement.setAttribute('width', `${event.size[0]}px`);
		this._svgElement.setAttribute('height', `${event.size[1]}px`);
		this._svgElement.setAttribute(
			'viewBox',
			`${event.viewport.physicalPosition[0]} ${event.viewport.physicalPosition[1]} ${event.viewport.physicalSize[0]} ${event.viewport.physicalSize[1]}`
		);
	}

	public clear(): void {
		this._svgElementMap.clear();
		const svgElement = document.getElementById(this._svgElementId);
		if (svgElement != null) {
			this._domElement.removeChild(svgElement);
		}
	}

	public pointerEventToArbPoint(event: PointerEvent | { clientX: number; clientY: number }): Vec2 {
		return this.clientWindowPointToArbPoint([event.clientX, event.clientY]);
	}

	public clientWindowPointToArbPoint(clientPoint: Vec2): Vec2 {
		const rect = this._svgElement.getBoundingClientRect();

		const x = clientPoint[0] - rect.left;
		const y = clientPoint[1] - rect.top;

		return [x, y];
	}
}

export interface TSvgBuilderOptions {
	domElement?: HTMLElement;
	callbackBased?: boolean;
	interactive?: boolean;
}
