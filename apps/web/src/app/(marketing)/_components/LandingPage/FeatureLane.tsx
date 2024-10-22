import { motion } from 'framer-motion';
import React from 'react';
import { cn } from '@dyn/ui';

import { FeatureLaneItem, type TFeatureLaneItem } from './FeatureLaneItem';

export const FeatureLane: React.FC<TProps> = (props) => {
	const {
		items,
		leftToRight = false,
		duration = 60,
		backgroundColor,
		rotationInDeg = 0,
		className
	} = props;
	const from = leftToRight ? '-100%' : 0;
	const to = leftToRight ? 0 : '-100%';

	return (
		<div
			className={cn(
				'flex w-[200%] overflow-hidden border-b-2 border-t-2 border-black py-2 md:w-[150%] lg:w-[120%]',
				className
			)}
			style={{ backgroundColor, transform: `rotate(${rotationInDeg.toString()}deg)` }}
		>
			<motion.div
				initial={{ x: from }}
				animate={{ x: to }}
				transition={{ duration, repeat: Infinity, ease: 'linear' }}
				className="flex w-full flex-shrink-0 justify-between"
			>
				{items.map((item, index) => (
					<FeatureLaneItem key={`${index.toString()}-${item.type}`} item={item} index={index} />
				))}
				<div />
			</motion.div>

			<motion.div
				initial={{ x: from }}
				animate={{ x: to }}
				transition={{ duration, repeat: Infinity, ease: 'linear' }}
				className="flex w-full flex-shrink-0 justify-between"
			>
				{items.map((item, index) => (
					<FeatureLaneItem key={index} item={item} index={index} />
				))}
				<div />
			</motion.div>
		</div>
	);
};
interface TProps {
	items: TFeatureLaneItem[];
	leftToRight?: boolean;
	duration?: number;
	backgroundColor: `#${string}`;
	rotationInDeg?: number;
	className?: string;
}
