import React from 'react';

export const FeatureLaneItem: React.FC<TProps> = (props) => {
	const { item, index } = props;

	switch (item.type) {
		case 'Text':
			return (
				<div
					key={`${index}-${item.type}`}
					className="flex items-center justify-center rounded-full bg-black px-3 py-1 text-white"
				>
					{item.text}
				</div>
			);
		case 'Icon':
			return (
				<div key={`${index}-${item.type}`} className="flex items-center gap-4 text-white">
					<div className="flex h-10 w-10 items-center justify-center rounded-full border border-black bg-gray-300">
						<span className="text-black">{item.icon}</span>
					</div>
					<span className="text-xl text-black">{item.name}</span>
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
