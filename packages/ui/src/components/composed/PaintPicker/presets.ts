import type { TGradientPaint, TSolidPaint } from './types';

export const SOLID_COLORS: TSolidPaint[] = [
	{ type: 'Solid', color: [226, 226, 226] }, // #E2E2E2
	{ type: 'Solid', color: [255, 117, 195] }, // #ff75c3
	{ type: 'Solid', color: [255, 166, 71] }, // #ffa647
	{ type: 'Solid', color: [255, 232, 63] }, // #ffe83f
	{ type: 'Solid', color: [159, 255, 91] }, // #9fff5b
	{ type: 'Solid', color: [112, 226, 255] }, // #70e2ff
	{ type: 'Solid', color: [205, 147, 255] }, // #cd93ff
	{ type: 'Solid', color: [9, 32, 63] } // #09203f
];

export const GRADIENT_COLORS: TGradientPaint[] = [
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [172, 203, 238] }, // #accbee
			{ position: 1, color: [231, 240, 253] } // #e7f0fd
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [213, 212, 208] }, // #d5d4d0
			{ position: 1, color: [238, 238, 236] } // #eeeeec
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [0, 0, 0] }, // #000000
			{ position: 1, color: [67, 67, 67] } // #434343
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [9, 32, 63] }, // #09203f
			{ position: 1, color: [83, 120, 149] } // #537895
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [172, 50, 228] }, // #AC32E4
			{ position: 1, color: [121, 24, 242] }, // #7918F2
			{ position: 1, color: [72, 1, 255] } // #4801FF
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [249, 83, 198] }, // #f953c6
			{ position: 1, color: [185, 29, 115] } // #b91d73
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [238, 9, 121] }, // #ee0979
			{ position: 1, color: [255, 106, 0] } // #ff6a00
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [240, 0, 0] }, // #F00000
			{ position: 1, color: [220, 40, 30] } // #DC281E
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [0, 198, 255] }, // #00c6ff
			{ position: 1, color: [0, 114, 255] } // #0072ff
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [79, 172, 254] }, // #4facfe
			{ position: 1, color: [0, 242, 254] } // #00f2fe
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [11, 163, 96] }, // #0ba360
			{ position: 1, color: [60, 186, 146] } // #3cba92
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [253, 252, 71] }, // #FDFC47
			{ position: 1, color: [36, 254, 65] } // #24FE41
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [138, 43, 226] }, // #8a2be2
			{ position: 0.33, color: [0, 0, 205] }, // #0000cd
			{ position: 0.66, color: [34, 139, 34] }, // #228b22
			{ position: 1, color: [204, 255, 0] } // #ccff00
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [64, 224, 208] }, // #40E0D0
			{ position: 0.5, color: [255, 140, 0] }, // #FF8C00
			{ position: 1, color: [255, 0, 128] } // #FF0080
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [252, 197, 228] }, // #fcc5e4
			{ position: 0.2, color: [253, 163, 75] }, // #fda34b
			{ position: 0.4, color: [255, 120, 130] }, // #ff7882
			{ position: 0.6, color: [200, 105, 158] }, // #c8699e
			{ position: 0.8, color: [112, 70, 170] }, // #7046aa
			{ position: 1, color: [12, 29, 184] }, // #0c1db8
			{ position: 1, color: [2, 15, 117] } // #020f75
		]
	},
	{
		type: 'Gradient',
		variant: { type: 'Linear' },
		stops: [
			{ position: 0, color: [255, 117, 195] }, // #ff75c3
			{ position: 0.2, color: [255, 166, 71] }, // #ffa647
			{ position: 0.4, color: [255, 232, 63] }, // #ffe83f
			{ position: 0.6, color: [159, 255, 91] }, // #9fff5b
			{ position: 0.8, color: [112, 226, 255] }, // #70e2ff
			{ position: 1, color: [205, 147, 255] } // #cd93ff
		]
	}
];
