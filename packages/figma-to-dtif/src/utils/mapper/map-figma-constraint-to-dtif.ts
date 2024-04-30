import type { COMP } from '@dyn/dtif-comp';

export function mapFigmaConstraintToDtif(figmaConstraint?: ConstraintType): COMP.AlignItems {
	switch (figmaConstraint) {
		case 'MIN':
			return 'Start';
		case 'MAX':
			return 'End';
		case 'CENTER':
			return 'Center';
		case 'STRETCH':
			return 'Stretch';
		default:
			// Fallback for unmatched or undefined constraint
			return 'Start';
	}
}
