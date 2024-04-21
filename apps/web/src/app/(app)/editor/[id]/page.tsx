import { promises as fs } from 'node:fs';
import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import { Editor } from '@/components';

const Page = async (props: TProps): Promise<React.ReactNode> => {
	const {
		params: { id }
	} = props;
	const defaultDtif = JSON.parse(
		await fs.readFile(`${process.cwd()}/public/templates/${id}.json`, 'utf8')
	) as COMP.DtifComposition;

	return <Editor dtif={defaultDtif} />;
};

export default Page;

interface TProps {
	params: { id: string };
}
