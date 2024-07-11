import React from 'react';
import { isDtif, isMdtif, prepareDtif } from '@dyn/arb-dtif';

import { EditorWrapper } from './_components';
import { getStaticDtif } from './_helper';

const Page = async (props: TProps): Promise<React.ReactNode> => {
	const {
		params: { id }
	} = props;
	const dtif = await getStaticDtif(id);

	if (isMdtif(dtif)) {
		const preparedDtif = await prepareDtif(dtif);
		return <EditorWrapper dtif={preparedDtif} />;
	}

	if (isDtif(dtif)) {
		const preparedDtif = await prepareDtif(dtif);
		return <EditorWrapper dtif={preparedDtif} />;
	}

	return (
		<div>
			<p>Not found</p>
		</div>
	);
};

export default Page;

interface TProps {
	params: { id: string };
}
