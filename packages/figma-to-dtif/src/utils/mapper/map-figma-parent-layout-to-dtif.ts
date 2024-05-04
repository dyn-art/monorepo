import type { COMP } from '@dyn/dtif-comp';

export function mapFigmaParentLayoutToDtif(
	node: FrameNode | ComponentNode | InstanceNode
): COMP.StaticLayoutParent | null {
	const flexDirection = mapLayoutModeToDtif(node.layoutMode);
	if (flexDirection == null) {
		return null;
	}

	return {
		flexDirection,
		alignItems: mapCounterAxisAlignItemsToDtif(node.counterAxisAlignItems),
		justifyContent: mapPrimaryAxisAlignItemsToDtif(node.primaryAxisAlignItems),
		padding: {
			top: { type: 'Abs', value: node.paddingRight },
			bottom: { type: 'Abs', value: node.paddingBottom },
			left: { type: 'Abs', value: node.paddingLeft },
			right: { type: 'Abs', value: node.paddingRight }
		},
		gap:
			flexDirection === 'Column' || flexDirection === 'ColumnReverse'
				? { x: { type: 'Abs', value: 0 }, y: { type: 'Abs', value: node.itemSpacing } }
				: { x: { type: 'Abs', value: node.itemSpacing }, y: { type: 'Abs', value: 0 } }
	};
}

function mapLayoutModeToDtif(
	layoutMode: 'NONE' | 'HORIZONTAL' | 'VERTICAL'
): COMP.FlexDirection | null {
	switch (layoutMode) {
		case 'HORIZONTAL':
			return 'Row';
		case 'VERTICAL':
			return 'Column';
		case 'NONE':
			return null;
	}
}

function mapCounterAxisAlignItemsToDtif(
	counterAxisAlignItems: 'MIN' | 'MAX' | 'CENTER' | 'BASELINE'
): COMP.AlignItems {
	switch (counterAxisAlignItems) {
		case 'MIN':
			return 'Start';
		case 'CENTER':
			return 'Center';
		case 'MAX':
			return 'End';
		case 'BASELINE':
			return 'Baseline';
	}
}

function mapPrimaryAxisAlignItemsToDtif(
	primaryAxisAlignItems: 'MIN' | 'MAX' | 'CENTER' | 'SPACE_BETWEEN'
): COMP.AlignContent {
	switch (primaryAxisAlignItems) {
		case 'MIN':
			return 'Start';
		case 'CENTER':
			return 'Center';
		case 'MAX':
			return 'End';
		case 'SPACE_BETWEEN':
			return 'SpaceBetween';
	}
}