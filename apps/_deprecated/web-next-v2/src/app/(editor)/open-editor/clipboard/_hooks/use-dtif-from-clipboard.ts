import { useQuery } from '@tanstack/react-query';
import { isDtif, isMdtif, type ARB, type TMdtifArtboard } from '@dyn/arb-dtif';

export function useDtifFromClipboard(): {
	data?: ARB.DtifArtboard | TMdtifArtboard;
	isLoading: boolean;
} {
	const { data, isLoading } = useQuery({
		queryKey: ['clipboard-dtif'],
		queryFn: async () => {
			let dtif: ARB.DtifArtboard | TMdtifArtboard | undefined;

			// Try to load Dtif from clipboard
			try {
				const text = await navigator.clipboard.readText();
				const maybeDtif: unknown = JSON.parse(text);
				if (isMdtif(maybeDtif) || isDtif(maybeDtif)) {
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
