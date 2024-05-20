import { toFunction, type TJsonFunction } from '@dyn/utils';

export async function runJsonFunction<GResponse>(
	jsonFunction: TJsonFunction,
	args: unknown[],
	env: 'iframe' | 'direct'
): Promise<GResponse> {
	if (env === 'direct') {
		const func = toFunction(jsonFunction);
		return Promise.resolve(func(...args));
	}

	return new Promise((resolve, reject) => {
		// Create the iframe element
		const iframe = document.createElement('iframe');
		iframe.style.display = 'none';
		document.body.appendChild(iframe);

		// Function to handle messages from the iframe
		const handleMessage = (event: MessageEvent): void => {
			if (event.source === iframe.contentWindow) {
				window.removeEventListener('message', handleMessage);
				document.body.removeChild(iframe);
				if (event.data.error) {
					reject(new Error(event.data.error));
				} else {
					resolve(event.data.result);
				}
			}
		};

		window.addEventListener('message', handleMessage);

		// Function to be executed inside the iframe
		const scriptContent = `
			window.addEventListener('message', (event) => {
				if (event.source !== window.parent) return;
				const { jsonFunction, args } = event.data;
				try {
					const func = new Function(...jsonFunction.args, jsonFunction.body);
					const result = func(...args);
					window.parent.postMessage({ result }, '*');
				} catch (error) {
					window.parent.postMessage({ error: error.message }, '*');
				}
			});
		`;

		// Inject the script into the iframe
		const iframeDoc = iframe.contentDocument || iframe.contentWindow?.document;
		if (iframeDoc != null) {
			const scriptElement = iframeDoc.createElement('script');
			scriptElement.type = 'text/javascript';
			scriptElement.text = scriptContent;
			iframeDoc.body.appendChild(scriptElement);

			// Send the function and args to be executed to the iframe
			iframe.contentWindow?.postMessage(
				{
					jsonFunction,
					args
				},
				'*'
			);
		} else {
			document.body.removeChild(iframe);
			reject(new Error('Failed to access iframe document'));
		}
	});
}
