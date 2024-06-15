import { useQuery } from '@tanstack/react-query';
import { prepareDtif, type ARB } from '@dyn/arb-dtif';

export function usePreparedDtif(dtif?: ARB.DtifArtboard): {
	data?: ARB.DtifArtboard;
	isLoading: boolean;
} {
	const { data, isLoading } = useQuery({
		queryKey: ['prepare-dtif', dtif],
		queryFn: async () => {
			return dtif != null ? prepareDtif(dtif) : undefined;
		},
		refetchOnWindowFocus: false
	});

	return { data, isLoading };
}
