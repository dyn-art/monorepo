import { createTransformMatrix, type COMP } from '@dyn/dtif';
import { BannerEditor } from '@/components';

const WIDTH = 1500;
const HEIGHT = 500;
const DTIF: COMP.DTIFComposition = {
	version: '0.0.1',
	name: 'Test',
	width: WIDTH,
	height: HEIGHT,
	rootNodeId: 0,
	nodes: {
		0: {
			type: 'Frame',
			children: [1],
			dimension: {
				width: WIDTH,
				height: HEIGHT
			},
			relativeTransform: createTransformMatrix(0, 0, 0),
			fill: {
				paintIds: [5]
			}
		},
		1: {
			type: 'Rectangle',
			dimension: {
				width: 100,
				height: 100
			},
			relativeTransform: createTransformMatrix((WIDTH - 100) / 2, (HEIGHT - 100) / 2, 30),
			rectangleCornerMixin: {
				bottomLeftRadius: 20
			},
			fill: {
				paintIds: [6]
			}
		}
	},
	paints: {
		5: {
			type: 'Solid',
			color: [189, 183, 107]
		},
		6: {
			type: 'Solid',
			color: [0, 0, 0]
		}
	},
	fonts: {},
	changes: [
		// {
		// 	type: 'EntityMoved',
		// 	entity: 1,
		// 	dx: 100,
		// 	dy: -300
		// }
	]
};

const Page: React.FC = () => {
	return (
		<div>
			<BannerEditor dtif={DTIF} height={HEIGHT} width={WIDTH} />
		</div>
	);
};

export default Page;
