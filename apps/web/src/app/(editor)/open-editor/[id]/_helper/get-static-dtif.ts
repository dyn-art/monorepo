import type { CNV } from '@dyn/cnv-dtif';
import { appFetchClient } from '@/core';

export async function getStaticDtif(id: string): Promise<unknown> {
	try {
		const result = await appFetchClient.get<CNV.DtifCanvas>(`templates/dtif/${id}.json`);
		return result.unwrap().data;
	} catch (e) {
		return null;
	}
}
