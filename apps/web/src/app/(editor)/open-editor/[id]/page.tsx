import React from 'react';
import { isDtifComposition, isMdtifComposition, prepareDtifComposition } from '@dyn/comp-dtif';

import { EditorWrapper } from './_components';
import { getStaticDtif } from './_helper';

const Page = async (props: TProps): Promise<React.ReactNode> => {
	const {
		params: { id }
	} = props;
	const dtif = await getStaticDtif(id);

	if (isDtifComposition(dtif)) {
		const preparedDtif = await prepareDtifComposition(dtif);
		return <EditorWrapper dtif={preparedDtif} />;
	}

	if (isMdtifComposition(dtif)) {
		const preparedDtif = await prepareDtifComposition(dtif.template);
		return (
			<EditorWrapper
				dtif={{
					...dtif,
					template: preparedDtif
				}}
			/>
		);
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
