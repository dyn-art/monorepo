import { promises as fs } from 'node:fs';
import { redirect } from 'next/navigation';
import type { COMP } from '@dyn/dtif-comp';

export async function getStaticDtif(id: string): Promise<unknown> {
	try {
		return JSON.parse(
			await fs.readFile(`${process.cwd()}/public/templates/dtif/${id}.json`, 'utf8')
		) as COMP.DtifComposition;
	} catch (e) {
		redirect('/editor/default');
	}
}
