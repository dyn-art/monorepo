import type { ARB } from '../../arb';

export function toKeyCode(keyCode: string): ARB.KeyCode {
	switch (keyCode) {
		case 'MetaLeft':
		case 'MetaRight':
		case 'Meta':
			return 'Meta';
		default:
			return keyCode as ARB.KeyCode;
	}
}
