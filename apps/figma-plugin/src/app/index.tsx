import React from 'react';
import { createRoot } from 'react-dom/client';

import App from './App';

const _ = React; // Ensure React is seen as "used" by auto-import plugin

function init() {
	const appContainer = document.getElementById('root');
	if (appContainer == null) {
		throw new Error('Can not find Root!');
	}
	const root = createRoot(appContainer);
	// https://forum.figma.com/t/how-to-work-with-react-router-dom-in-figma-plugin/2450/9
	root.render(<App />);
}

// The 'DOMContentLoaded' event listener ensures that the 'init' function is called
// only after the entire HTML document has been loaded and parsed, making sure
// the DOM elements are ready to be manipulated.
document.addEventListener('DOMContentLoaded', init);
