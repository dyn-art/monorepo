import type { COMP } from '../../comp';

export function toKeyCode(keyCode: string): COMP.KeyCode {
	switch (keyCode) {
		case 'MetaLeft':
		case 'MetaRight':
		case 'Meta':
			return 'Meta';
		default:
			return keyCode as COMP.KeyCode;
	}
}
