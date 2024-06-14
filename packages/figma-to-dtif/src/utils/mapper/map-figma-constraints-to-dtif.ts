import type { CNV } from '@dyn/cnv-dtif';

import { mapFigmaConstraintToDtif } from './map-figma-constraint-to-dtif';

export function mapFigmaConstraintsToDtif(figmaConstraints?: Constraints): CNV.Constraints {
	return {
		horizontal: mapFigmaConstraintToDtif(figmaConstraints?.horizontal),
		vertical: mapFigmaConstraintToDtif(figmaConstraints?.vertical)
	};
}
