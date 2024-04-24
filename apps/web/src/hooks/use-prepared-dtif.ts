import { useQuery } from '@tanstack/react-query';
import { prepareDtifComposition, type COMP } from '@dyn/dtif-comp';

export function usePreparedDtif(dtif?: COMP.DtifComposition): {
	data?: COMP.DtifComposition;
	isLoading: boolean;
} {
	const { data, isLoading } = useQuery({
		queryKey: ['prepare-dtif', dtif],
		queryFn: async () => {
			return dtif != null ? prepareDtifComposition(dtif) : undefined;
		},
		refetchOnWindowFocus: false
	});

	return { data, isLoading };
}
