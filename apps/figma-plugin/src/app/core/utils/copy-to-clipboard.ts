/**
 * Unsecured fallback for copying text to clipboard
 * @param text - The text to be copied to the clipboard
 */
function unsecuredCopyToClipboard(text: string) {
	// Create a textarea element
	const textArea = document.createElement('textarea');

	// Add styles to make the textarea invisible and avoid scrolling
	textArea.style.position = 'fixed';
	textArea.style.top = '0';
	textArea.style.left = '0';
	textArea.style.width = '2em';
	textArea.style.height = '2em';
	textArea.style.padding = '0';
	textArea.style.border = 'none';
	textArea.style.outline = 'none';
	textArea.style.boxShadow = 'none';
	textArea.style.background = 'transparent';

	// Apply text to text area and add it to the DOM
	textArea.value = text;
	document.body.appendChild(textArea);

	// Focus and select the textarea content
	textArea.focus();
	textArea.select();

	// Attempt to copy the text to the clipboard
	try {
		document.execCommand('copy');
	} catch (e) {
		// do nothing
	}

	// Remove the textarea element from the DOM
	document.body.removeChild(textArea);
}

/**
 * Copies the text passed as param to the system clipboard
 * Check if using HTTPS and navigator.clipboard is available
 * Then uses standard clipboard API, otherwise uses fallback
 *
 * Inspired by: https://stackoverflow.com/questions/71873824/copy-text-to-clipboard-cannot-read-properties-of-undefined-reading-writetext
 * and https://forum.figma.com/t/write-to-clipboard-from-custom-plugin/11860/12
 *
 * @param content - The content to be copied to the clipboard
 */
export async function copyToClipboard(content: string): Promise<void> {
	// If the context is secure and clipboard API is available, use it
	if (window.isSecureContext && typeof navigator.clipboard.writeText === 'function') {
		await navigator.clipboard.writeText(content);
	}
	// Otherwise, use the unsecured fallback
	else {
		unsecuredCopyToClipboard(content);
	}
}
