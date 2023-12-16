import React from 'react';
import { useNavigate } from 'react-router-dom';

import { Footer, Navbar } from '../components';

const Settings: React.FC = () => {
	const navigate = useNavigate();

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
			<Footer leftContent={{ variant: 'version' }} rightContent={{ variant: 'socials' }} />
		</>
	);
};

export default Settings;
