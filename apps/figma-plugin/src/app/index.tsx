import React from 'react';
import ReactDOM from 'react-dom/client';

import { App } from './App';

function init(): void {
	ReactDOM.createRoot(document.getElementById('root') as any).render(
		<React.StrictMode>
			<App />
		</React.StrictMode>
	);
}

document.addEventListener('DOMContentLoaded', init);
