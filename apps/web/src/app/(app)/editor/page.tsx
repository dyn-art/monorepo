import { promises as fs } from 'node:fs';
import type { COMP } from '@dyn/dtif-comp';
import { Editor } from '@/components';

const Page = async (): Promise<React.ReactNode> => {
	const defaultDtif = JSON.parse(
		await fs.readFile(`${process.cwd()}/public/templates/default.json`, 'utf8')
	) as COMP.DtifComposition;

	return <Editor dtif={defaultDtif} />;
};

export default Page;
