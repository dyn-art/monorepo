import { useQuery } from '@tanstack/react-query';
import { isDtifComposition, type COMP } from '@dyn/comp-dtif';

export function useDtifFromClipboard(defaultDtif: COMP.DtifComposition): {
	data?: COMP.DtifComposition;
	isLoading: boolean;
} {
	const { data, isLoading } = useQuery({
		queryKey: ['prepare-dtif'],
		queryFn: async () => {
			try {
				const text = await navigator.clipboard.readText();
				const maybeDtif: unknown = JSON.parse(text);
				if (isDtifComposition(maybeDtif)) {
					console.log('Loaded DTIF from Clipboard: ', maybeDtif);
					return maybeDtif;
				}
				console.warn('Invalid DTIF from Clipboard!');
			} catch (e) {
				console.warn('Failed to load DTIF from Clipboard!');
			}
			return defaultDtif;
		}
	});

	return { data, isLoading };
}
