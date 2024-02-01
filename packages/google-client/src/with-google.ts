import {
	hasFeatures,
	RequestException,
	type TEnforceFeatures,
	type TFeatureKeys,
	type TFetchClient,
	type TSelectFeatures
} from '@dyn/fetch-client';

import type { paths } from './gen/v1';

export function withGoogle<GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<TEnforceFeatures<GSelectedFeatureKeys, ['base', 'openapi']>>
): TFetchClient<['google', ...GSelectedFeatureKeys], paths> {
	if (hasFeatures(fetchClient, ['openapi'])) {
		fetchClient._features.push('google');

		const googleFeature: TSelectFeatures<['google']> = {
			async getWebFonts(this: TFetchClient<['base', 'openapi', 'google'], paths>, options = {}) {
				return this.get('/webfonts', {
					queryParams: {
						key: 'not-set', // Set by middleware,
						...options
					}
				});
			},
			async getFontFileURL(
				this: TFetchClient<['base', 'openapi', 'google'], paths>,
				family,
				options = {}
			) {
				const { fontWeight = 400, fontStyle = 'regular', capability = 'VF' } = options;
				const searchResult = await this.getWebFonts({
					capability,
					family
				});

				if (searchResult.isErr()) {
					if (searchResult.error instanceof RequestException && searchResult.error.status === 404) {
						return null;
					}
					throw searchResult.error;
				}

				// Find font family
				const items = searchResult.value.data.items ?? [];
				const font = items.find((f) => f.family === family);
				if (font == null) {
					return null;
				}

				// Find the closest match for font weight and style
				const closestVariant = findClosestVariant(font.variants ?? [], fontWeight, fontStyle);

				// Find font file URL
				if (font.files != null && closestVariant != null) {
					return font.files[closestVariant] ?? null;
				}

				return null;
			}
		};

		// Merge existing features from the state with the new api feature
		const _fetchClient = Object.assign(fetchClient, googleFeature);

		return _fetchClient as TFetchClient<['google', ...GSelectedFeatureKeys], paths>;
	}

	throw Error('FetchClient must have "openapi" feature to use withGoogle');
}

function findClosestVariant(
	variants: string[],
	desiredWeight: number,
	desiredStyle: string
): string | null {
	const styleSuffix = desiredStyle.toLowerCase() === 'italic' ? 'italic' : '';
	const desiredVariant = `${desiredWeight}${styleSuffix}`;
	if (variants.includes(desiredVariant)) {
		return desiredVariant;
	}

	// Fallback to regular if desired variant is not available
	return variants.includes('regular') ? 'regular' : null;
}
