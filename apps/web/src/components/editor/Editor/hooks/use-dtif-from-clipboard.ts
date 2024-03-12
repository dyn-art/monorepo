import { useQuery } from '@tanstack/react-query';
import { isDtifComposition, prepareDtifComposition, type COMP } from '@dyn/comp-dtif';

export function useDtifFromClipboard(defaultDtif: COMP.DtifComposition): {
	data?: COMP.DtifComposition;
	isLoading: boolean;
} {
	const { data, isLoading } = useQuery({
		queryKey: ['prepare-dtif'],
		queryFn: async () => {
			let dtif: COMP.DtifComposition = defaultDtif;
			let loadedFromClipboard = false;

			// Try to load DTIF from clipboard
			try {
				const text = await navigator.clipboard.readText();
				const maybeDtif: unknown = JSON.parse(text);
				if (isDtifComposition(maybeDtif)) {
					dtif = maybeDtif;
					loadedFromClipboard = true;
				}
			} catch (e) {
				console.warn('Failed to load DTIF from Clipboard!');
			}

			// Prepare DTIF by loading assets, ..
			const preparedDtif = await prepareDtifComposition(dtif);
			console.log(
				loadedFromClipboard ? 'Loaded DTIF from Clipboard: ' : 'Loaded default DTIF: ',
				preparedDtif
			);

			return preparedDtif;
		}
	});

	return { data, isLoading };
}
