import type { COMP } from '@dyn/dtif-comp';

import { mapFigmaConstraintToDtif } from './map-figma-constraint-to-dtif';

export function mapFigmaConstraintsToDtif(figmaConstraints?: Constraints): COMP.Constraints {
	return {
		horizontal: mapFigmaConstraintToDtif(figmaConstraints?.horizontal),
		vertical: mapFigmaConstraintToDtif(figmaConstraints?.vertical)
	};
}
