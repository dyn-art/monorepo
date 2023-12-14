import React from 'react';

export function useWatchMode(currentEnvironment: EEnvironment): { isConnected: boolean } {
	const [isConnected, setIsConnected] = React.useState(false);
	const ws = React.useRef<WebSocket | null>(null);
	const isAttemptingReconnect = React.useRef(false);
	const reconnectTimeout = React.useRef<NodeJS.Timeout | null>(null);

	// Open WebSocket connection
	React.useEffect(() => {
		// Helper function to open a WebSocket connection
		const openWebsocket = (): void => {
			isAttemptingReconnect.current = true;
			const newWS = new WebSocket('ws://localhost:9001/ws');

			newWS.onopen = () => {
				ws.current = newWS;
				setIsConnected(true);
				isAttemptingReconnect.current = false;
			};

			newWS.onclose = () => {
				setIsConnected(false);
				reconnectTimeout.current = setTimeout(openWebsocket, 3000);
			};
		};

		if (ws.current == null && !isConnected && !isAttemptingReconnect.current) {
			openWebsocket();
		}

		// Cleanup WebSocket connection
		return () => {
			ws.current?.close();
			if (reconnectTimeout.current != null) {
				clearTimeout(reconnectTimeout.current);
			}
		};
	}, []);

	// Handle WebSocket proxy server messages
	React.useEffect(() => {
		// Helper function to process WebSocket messages
		const onServerMessage = (event: MessageEvent): void => {
			try {
				const message = JSON.parse(event.data);
				const rawData = message?.data;
				if (message?.type !== 'server.broadcast' || rawData == null) {
					return;
				}

				const pluginMessage = JSON.parse(String.fromCharCode(...rawData.data));
				const target = currentEnvironment === EEnvironment.APP ? window.parent : window;
				target.postMessage({ pluginMessage, environment: currentEnvironment }, '*');
			} catch (error) {
				console.error('Error processing message from server', error);
			}
		};

		// Register WebSocket message event listener
		if (ws.current != null) {
			ws.current.onmessage = onServerMessage;
		}

		// Cleanup WebSocket message event listener
		return () => {
			if (ws.current != null) {
				ws.current.onmessage = null;
			}
		};
	}, [ws.current, currentEnvironment]);

	// Handle window messages
	React.useEffect(() => {
		// Helper function to process Window messages
		const onWindowMessage = (event: MessageEvent): void => {
			const eventData = event.data;
			if (eventData?.pluginMessage != null && eventData?.environment !== currentEnvironment) {
				if (ws.current?.readyState === WebSocket.OPEN) {
					ws.current.send(JSON.stringify(eventData.pluginMessage));
				}
			}
		};

		// Register Window message event listener
		const target = currentEnvironment === EEnvironment.APP ? window : window.parent;
		target.addEventListener('message', onWindowMessage);

		// Cleanup Window message event listener
		return () => {
			target.removeEventListener('message', onWindowMessage);
		};
	}, [ws.current, currentEnvironment]);

	return { isConnected };
}

export enum EEnvironment {
	APP,
	WEB
}
