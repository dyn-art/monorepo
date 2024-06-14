import type { CNV } from '../../cnv';

export function toMouseButton(button: number): CNV.MouseButton {
	switch (button) {
		case 0:
			return 'Left';
		case 1:
			return 'Middle';
		case 2:
			return 'Right';
		default:
			return { Other: button };
	}
}
