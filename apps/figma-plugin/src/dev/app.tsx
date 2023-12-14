import React from 'react';
import ReactDOM from 'react-dom/client';

import { EEnvironment, useWatchMode } from './useWatchMode';

const PreviewApp: React.FC = () => {
	const { isConnected } = useWatchMode(EEnvironment.APP);

	return (
		<div
			style={{
				display: 'flex',
				justifyContent: 'center',
				alignItems: 'center',
				width: '100%',
				height: '100%',
				backgroundColor: isConnected ? '#4ade80' : '#f87171'
			}}
		/>
	);
};

function init(): void {
	ReactDOM.createRoot(document.getElementById('root') as any).render(
		<React.StrictMode>
			<PreviewApp />
		</React.StrictMode>
	);
}

document.addEventListener('DOMContentLoaded', init);
