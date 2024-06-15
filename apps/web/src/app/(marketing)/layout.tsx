import { LayoutWrapper } from '@dyn/ui';

import { Navbar } from './_components';

const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return (
		<div className="bg-[#FCFAF4]">
			<div className="relative isolate">
				<svg
					className="absolute inset-x-0 top-0 -z-10 h-[64rem] w-full stroke-gray-200 [mask-image:radial-gradient(32rem_32rem_at_center,white,transparent)]"
					aria-hidden="true"
				>
					<defs>
						<pattern
							id="1f932ae7-37de-4c0a-a8b0-a6e3b4d44b84"
							width={200}
							height={200}
							x="50%"
							y={-1}
							patternUnits="userSpaceOnUse"
						>
							<path d="M.5 200V.5H200" fill="none" />
						</pattern>
					</defs>
					<svg x="50%" y={-1} className="overflow-visible fill-gray-50">
						<path
							d="M-200 0h201v201h-201Z M600 0h201v201h-201Z M-400 600h201v201h-201Z M200 800h201v201h-201Z"
							strokeWidth={0}
						/>
					</svg>
					<rect
						width="100%"
						height="100%"
						strokeWidth={0}
						fill="url(#1f932ae7-37de-4c0a-a8b0-a6e3b4d44b84)"
					/>
				</svg>
				<div
					className="absolute left-1/2 right-0 top-0 -z-10 -ml-24 transform-gpu overflow-hidden blur-3xl lg:ml-24 xl:ml-48"
					aria-hidden="true"
				>
					<div
						className="aspect-[801/1036] w-[50.0625rem] bg-gradient-to-tr from-[#FE3863] to-[#FCB600] opacity-30"
						style={{
							clipPath:
								'polygon(63.1% 29.5%, 100% 17.1%, 76.6% 3%, 48.4% 0%, 44.6% 4.7%, 54.5% 25.3%, 59.8% 49%, 55.2% 57.8%, 44.4% 57.2%, 27.8% 47.9%, 35.1% 81.5%, 0% 97.7%, 39.2% 100%, 35.2% 81.4%, 97.2% 52.8%, 63.1% 29.5%)'
						}}
					/>
				</div>
				<LayoutWrapper size="full">
					<Navbar />
					{children}
				</LayoutWrapper>
			</div>
		</div>
	);
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
