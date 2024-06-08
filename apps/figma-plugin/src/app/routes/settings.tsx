import { Button } from '@dyn/ui';
import { createState, withUndo } from 'feature-state';
import { useGlobalState } from 'feature-state-react';
import React from 'react';
import { useNavigate } from 'react-router-dom';

import { Footer, Navbar } from '../components';

const $myState = withUndo(createState(0));

const Settings: React.FC = () => {
	const navigate = useNavigate();
	const myState = useGlobalState($myState);

	return (
		<>
			<Navbar
				leftContent={{
					variant: 'back',
					onClick: () => {
						navigate(-1);
					}
				}}
				centerText="Settings"
				rightContent={{ variant: 'info', url: 'todo' }}
			/>
			<p>Settings</p>
			<Button
				onClick={() => {
					$myState.set($myState.get() + 1);
				}}
			>
				{myState}
			</Button>
			<Button
				onClick={() => {
					$myState.undo();
				}}
			>
				Undo
			</Button>
			<Footer leftContent={{ variant: 'version' }} rightContent={{ variant: 'socials' }} />
		</>
	);
};

export default Settings;
