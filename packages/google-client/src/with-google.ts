import {
	createApiFetchClient,
	Err,
	hasFeatures,
	isStatusCode,
	Ok,
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
			rawFetchClient: createApiFetchClient(),
			async getWebFonts(this: TFetchClient<['base', 'openapi', 'google'], paths>, options = {}) {
				return this.get('/webfonts', {
					queryParams: {
						key: 'not-set', // Set by middleware,
						...options
					}
				});
			},
			async getFontFileUrl(
				this: TFetchClient<['base', 'openapi', 'google'], paths>,
				family,
				options = {}
			) {
				const { fontWeight = 400, fontStyle = 'regular', capability = 'VF' } = options;

				// Fetch web fonts
				const response = await this.getWebFonts({
					capability,
					family
				});
				if (response.isErr()) {
					if (isStatusCode(response.error, 404)) {
						return Ok(null);
					}
					return Err(response.error);
				}

				// Find the closest match for font family, weight and style
				const items = response.value.data.items ?? [];
				const font = items.find((f) => f.family === family);
				if (font == null) {
					return Ok(null);
				}
				const closestVariant = findClosestVariant(font.variants ?? [], fontWeight, fontStyle);

				// Find font file URL
				if (font.files != null && closestVariant != null) {
					const fileUrl = font.files[closestVariant];
					if (fileUrl != null) {
						return Ok(fileUrl.replace('http://', 'https://'));
					}
				}

				return Ok(null);
			},
			async downloadFontFile(
				this: TFetchClient<['base', 'openapi', 'google'], paths>,
				family,
				options = {}
			) {
				// Fetch font download url
				const downloadUrlResponse = await this.getFontFileUrl(family, options);
				if (downloadUrlResponse.isErr()) {
					return Err(downloadUrlResponse.error);
				}
				const downloadUrl = downloadUrlResponse.value;
				if (downloadUrl == null) {
					return Ok(null);
				}

				// Fetch font binary
				const response = await this.rawFetchClient.get(downloadUrl, { parseAs: 'arrayBuffer' });
				if (response.isErr()) {
					if (isStatusCode(response.error, 404)) {
						return Ok(null);
					}
					return Err(response.error);
				}

				return Ok(new Uint8Array(response.value.data));
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
