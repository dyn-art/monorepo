import React from 'react';

export const FeatureLaneItem: React.FC<TProps> = (props) => {
	const { item, index } = props;

	switch (item.type) {
		case 'Text':
			return (
				<div
					key={`${index}-${item.type}`}
					className="flex items-center justify-center rounded-full bg-black px-3 py-1 text-xs text-white sm:text-lg lg:text-xl"
				>
					{item.text}
				</div>
			);
		case 'Icon':
			return (
				<div key={`${index}-${item.type}`} className="flex items-center gap-2 text-white lg:gap-4">
					<div className="flex h-6 w-6 items-center justify-center rounded-full border border-black bg-gray-300 sm:h-10 sm:w-10">
						<span className="text-xs text-black sm:text-xl">{item.icon}</span>
					</div>
					<span className="text-lg text-black sm:text-lg lg:text-xl">{item.name}</span>
				</div>
			);
	}
};

export type TFeatureLaneItem =
	| { type: 'Text'; text: string }
	| { type: 'Icon'; icon: string; name: string };

interface TProps {
	item: TFeatureLaneItem;
	index: number;
}
