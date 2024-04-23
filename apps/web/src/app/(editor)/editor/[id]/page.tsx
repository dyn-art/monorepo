import React from 'react';
import { isDtifComposition, isMdtifComposition, prepareDtifComposition } from '@dyn/dtif-comp';

import { UniversalEditor } from '../_components/UniversalEditor';
import { getStaticDtif } from './_helper/get-static-dtif';

const Page = async (props: TProps): Promise<React.ReactNode> => {
	const {
		params: { id }
	} = props;
	const dtif = await getStaticDtif(id);

	console.log('Loaded static DTIF: ', dtif);

	if (isDtifComposition(dtif)) {
		const preparedDtif = await prepareDtifComposition(dtif);
		return <UniversalEditor dtif={preparedDtif} />;
	}

	if (isMdtifComposition(dtif)) {
		const preparedDtif = await prepareDtifComposition(dtif.template);
		return (
			<UniversalEditor
				dtif={{
					...dtif,
					template: preparedDtif
				}}
			/>
		);
	}

	return null;
};

export default Page;

interface TProps {
	params: { id: string };
}
