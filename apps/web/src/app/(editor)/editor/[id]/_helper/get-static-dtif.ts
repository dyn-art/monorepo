import { promises as fs } from 'node:fs';
import type { COMP } from '@dyn/dtif-comp';

export async function getStaticDtif(id: string): Promise<unknown> {
	try {
		return JSON.parse(
			await fs.readFile(`${process.cwd()}/public/templates/dtif/${id}.json`, 'utf8')
		) as COMP.DtifComposition;
	} catch (e) {
		return null;
	}
}
