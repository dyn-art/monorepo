import React from 'react';
import ReactDOM from 'react-dom/client';

import { usePreview } from './usePreview';

const PreviewApp: React.FC = () => {
	const { isConnected } = usePreview(true);

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
