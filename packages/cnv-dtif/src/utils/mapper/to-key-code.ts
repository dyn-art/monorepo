import type { CNV } from '../../cnv';

export function toKeyCode(keyCode: string): CNV.KeyCode {
	switch (keyCode) {
		case 'MetaLeft':
		case 'MetaRight':
		case 'Meta':
			return 'Meta';
		default:
			return keyCode as CNV.KeyCode;
	}
}
