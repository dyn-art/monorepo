import React from 'react';
import { Card } from '@/components/layout';
import { Button, Separator } from '@/components/primitive';

const DTOM: React.FC = () => {
	const [canvasState, setCanvasState] = React.useState(CANVAS_STATE.NONE);

	return (
		<>
			<div className="space-y-1">
				<h4 className="text-sm font-medium leading-none">DTOM Playground</h4>
				<p className="text-sm text-muted-foreground">
					Test the Design Tree Object Model (DTOM) canvas engine.
				</p>
			</div>
			<Separator className="my-4" />
			<div className="flex h-5 items-center space-x-4 text-sm mb-4">
				<Button variant={'link'} onClick={() => setCanvasState(CANVAS_STATE.WIP)}>
					WIP
				</Button>
				<Separator orientation="vertical" />
				<Button variant={'link'} onClick={() => setCanvasState(CANVAS_STATE.MOVING_RECTS)}>
					Moving Rects
				</Button>
			</div>
			<Card>
				<div>Canvas Here</div>
			</Card>
		</>
	);
};

export default DTOM;

enum CANVAS_STATE {
	NONE,
	WIP,
	MOVING_RECTS
}
