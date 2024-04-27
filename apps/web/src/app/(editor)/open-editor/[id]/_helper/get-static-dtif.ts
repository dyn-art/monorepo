import type { COMP } from '@dyn/dtif-comp';
import { appFetchClient } from '@/core';

export async function getStaticDtif(id: string): Promise<unknown> {
	try {
		const result = await appFetchClient.get<COMP.DtifComposition>(`templates/dtif/${id}.json`);
		return result.unwrap().data;
	} catch (e) {
		return null;
	}
}
