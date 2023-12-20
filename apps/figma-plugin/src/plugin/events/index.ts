import documentChange from './document-change';
import intermediateFormatExport from './intermediate-format-export';
import selectionChangeEvent from './selection-change';
import uiRouteChange from './ui-route-change';

export const events = [
	uiRouteChange,
	selectionChangeEvent,
	documentChange,
	intermediateFormatExport
];
