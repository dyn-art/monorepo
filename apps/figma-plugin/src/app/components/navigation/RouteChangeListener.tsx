import React from 'react';
import { useLocation } from 'react-router-dom';

// Note: Encountered issues when implementing this logic as a standalone hook.
// Error: The specific error was 'VM5394:49 Uncaught SyntaxError: Unexpected token '&''.
export const RouteChangeListener: React.FC = () => {
	const location = useLocation();

	React.useEffect(() => {
		// TODO
	}, [location]);

	return null;
};
