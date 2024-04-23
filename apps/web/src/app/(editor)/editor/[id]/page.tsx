import { promises as fs } from 'node:fs';
import { redirect } from 'next/navigation';
import React from 'react';
import { isDtifComposition, isMdtifComposition, type COMP } from '@dyn/dtif-comp';
import { Container } from '@dyn/ui';
import { Editor } from '@/components';

const Page = async (props: TProps): Promise<React.ReactNode> => {
	const {
		params: { id }
	} = props;
	const dtif = await getStaticDtif(id);

	if (isDtifComposition(dtif)) {
		return (
			<Container className="h-screen" size="full" tag="main">
				<Editor dtif={dtif} />
			</Container>
		);
	}

	if (isMdtifComposition(dtif)) {
		return (
			<Container size="default" tag="main">
				<p>Hello World</p>
			</Container>
		);
	}

	return (
		<Container size="default" tag="main">
			<p>Not found</p>
		</Container>
	);
};

export default Page;

async function getStaticDtif(id: string): Promise<unknown> {
	try {
		return JSON.parse(
			await fs.readFile(`${process.cwd()}/public/templates/dtif/${id}.json`, 'utf8')
		) as COMP.DtifComposition;
	} catch (e) {
		redirect('/editor/default');
	}
}

interface TProps {
	params: { id: string };
}
