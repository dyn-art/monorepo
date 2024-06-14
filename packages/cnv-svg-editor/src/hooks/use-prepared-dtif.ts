import { useQuery } from '@tanstack/react-query';
import { prepareDtif, type CNV } from '@dyn/cnv-dtif';

export function usePreparedDtif(dtif?: CNV.DtifCanvas): {
	data?: CNV.DtifCanvas;
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
