import { useQuery } from '@tanstack/react-query';
import {
	isDtifComposition,
	isMdtifComposition,
	type COMP,
	type TMdtifComposition
} from '@dyn/comp-dtif';

export function useDtifFromClipboard(): {
	data?: COMP.DtifComposition | TMdtifComposition;
	isLoading: boolean;
} {
	const { data, isLoading } = useQuery({
		queryKey: ['clipboard-dtif'],
		queryFn: async () => {
			let dtif: COMP.DtifComposition | TMdtifComposition | undefined;

			// Try to load Dtif from clipboard
			try {
				const text = await navigator.clipboard.readText();
				const maybeDtif: unknown = JSON.parse(text);
				if (isDtifComposition(maybeDtif) || isMdtifComposition(maybeDtif)) {
					dtif = maybeDtif;
				}
			} catch (e) {
				console.warn('Failed to load DTIF from Clipboard by exception: ', e);
			}

			return dtif;
		},
		refetchOnWindowFocus: false
	});

	return { data, isLoading };
}
