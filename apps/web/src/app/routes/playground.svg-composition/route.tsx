import React from 'react';
import { Card, CONTENT_WIDTH, MaxWidthWrapper } from '@/components/layout';
import { FPSStats } from '@/components/monitoring';
import { Button, Separator } from '@/components/primitive';

import { Static } from './components';

const DTOM: React.FC = () => {
	const [canvasState, setCanvasState] = React.useState(CANVAS_STATE.NONE);

	return (
		<MaxWidthWrapper className="mt-8">
			<div className="flex flex-row justify-between align-top">
				<div className="space-y-1">
					<h4 className="text-sm font-medium leading-none">SVG Composition Playground</h4>
					<p className="text-muted-foreground text-sm">Test the SVG Composition canvas engine.</p>
				</div>
				<FPSStats />
			</div>
			<Separator className="my-4" />
			<div className="mb-4 flex h-5 items-center space-x-4 text-sm">
				<Button variant={'link'} onClick={() => setCanvasState(CANVAS_STATE.STATIC)}>
					Static
				</Button>
			</div>
			<Card
				style={{ maxWidth: CONTENT_WIDTH, height: CONTENT_WIDTH }}
				className="flex items-center justify-center"
			>
				{/* Static */}
				{canvasState === CANVAS_STATE.STATIC && <Static size={CONTENT_WIDTH} />}

				{/* None */}
				{canvasState === CANVAS_STATE.NONE && <p>Select Playground</p>}
			</Card>
		</MaxWidthWrapper>
	);
};

export default DTOM;

enum CANVAS_STATE {
	NONE,
	STATIC
}
