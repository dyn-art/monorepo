import { useQuery } from '@tanstack/react-query';
import { isDtif, isMdtif, type CNV, type TMdtifCanvas } from '@dyn/cnv-dtif';

export function useDtifFromClipboard(): {
	data?: CNV.DtifCanvas | TMdtifCanvas;
	isLoading: boolean;
} {
	const { data, isLoading } = useQuery({
		queryKey: ['clipboard-dtif'],
		queryFn: async () => {
			let dtif: CNV.DtifCanvas | TMdtifCanvas | undefined;

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
