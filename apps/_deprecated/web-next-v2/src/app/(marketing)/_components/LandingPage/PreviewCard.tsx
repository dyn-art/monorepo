import React from 'react';

export const PreviewCard = React.forwardRef<HTMLDivElement, TProps>((props, ref) => {
	const { src, dockingPointColor } = props;

	return (
		<div className="relative">
			<img
				src={src}
				alt=""
				className="aspect-[2/3] w-full rounded-xl bg-gray-900/5 object-cover shadow-lg"
			/>
			<div className="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-inset ring-gray-900/10" />
			<div
				className="absolute -bottom-2 left-0 right-0 mx-auto h-4 w-4 rounded-full border-2 border-[#FCFAF4]"
				style={{ backgroundColor: dockingPointColor }}
				ref={ref}
			/>
		</div>
	);
});
PreviewCard.displayName = 'PreviewCard';

interface TProps {
	src: string;
	dockingPointColor: `#${string}`;
}
