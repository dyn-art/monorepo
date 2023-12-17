import React from 'react';
import { useNavigate } from 'react-router-dom';
import { createState } from '@dyn/state';
import { Button } from '@dyn/ui';

import { Footer, Navbar } from '../components';
import { useDynState } from '../hooks';

const MY_STATE = createState(0);

const Settings: React.FC = () => {
	const navigate = useNavigate();
	const myState = useDynState(MY_STATE);

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
					MY_STATE.set(MY_STATE.get() + 1);
					console.log({ MY_STATE });
				}}
			>
				{myState}
			</Button>
			<Footer leftContent={{ variant: 'version' }} rightContent={{ variant: 'socials' }} />
		</>
	);
};

export default Settings;
