import type { ARB } from '@dyn/arb-dtif';
import { appFetchClient } from '@/core';

export async function getStaticDtif(id: string): Promise<unknown> {
	try {
		const result = await appFetchClient.get<ARB.DtifArtboard>(`templates/dtif/${id}.json`);
		return result.unwrap().data;
	} catch (e) {
		return null;
	}
}
