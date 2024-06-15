import type { ARB } from '@dyn/arb-dtif';

import { mapFigmaConstraintToDtif } from './map-figma-constraint-to-dtif';

export function mapFigmaConstraintsToDtif(figmaConstraints?: Constraints): ARB.Constraints {
	return {
		horizontal: mapFigmaConstraintToDtif(figmaConstraints?.horizontal),
		vertical: mapFigmaConstraintToDtif(figmaConstraints?.vertical)
	};
}
