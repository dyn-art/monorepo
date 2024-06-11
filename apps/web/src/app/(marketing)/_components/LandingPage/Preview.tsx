'use client';

import React from 'react';
import {
	AnimatedBeam,
	Circle,
	LogoIcon,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue
} from '@dyn/ui';

import { PreviewCard } from './PreviewCard';

export const Preview: React.FC = () => {
	const card1Ref = React.useRef<HTMLDivElement>(null);
	const card2Ref = React.useRef<HTMLDivElement>(null);
	const card3Ref = React.useRef<HTMLDivElement>(null);
	const card4Ref = React.useRef<HTMLDivElement>(null);
	const card5Ref = React.useRef<HTMLDivElement>(null);
	const nodeRef = React.useRef<HTMLDivElement>(null);
	const templateRef = React.useRef<HTMLDivElement>(null);
	const datasetRef = React.useRef<HTMLDivElement>(null);
	const containerRef = React.useRef<HTMLDivElement>(null);

	return (
		<div className="relative" ref={containerRef}>
			<AnimatedBeam
				containerRef={containerRef}
				fromRef={nodeRef}
				toRef={card1Ref}
				className="-z-10"
				duration={3}
				pathColor="#683DFD"
				pathWidth={4}
			/>
			<AnimatedBeam
				containerRef={containerRef}
				fromRef={nodeRef}
				toRef={card2Ref}
				className="-z-10"
				duration={3}
				pathColor="#E337FF"
				pathWidth={4}
			/>
			<AnimatedBeam
				containerRef={containerRef}
				fromRef={nodeRef}
				toRef={card3Ref}
				className="-z-10"
				duration={3}
				pathColor="#01D3B1"
				pathWidth={4}
			/>
			<AnimatedBeam
				containerRef={containerRef}
				fromRef={nodeRef}
				toRef={card4Ref}
				className="-z-10"
				duration={3}
				pathColor="#FE3863"
				pathWidth={4}
			/>
			<AnimatedBeam
				containerRef={containerRef}
				fromRef={nodeRef}
				toRef={card5Ref}
				className="-z-10"
				duration={3}
				pathColor="#FCB600"
				pathWidth={4}
			/>
			<AnimatedBeam
				containerRef={containerRef}
				fromRef={templateRef}
				toRef={nodeRef}
				className="-z-10"
				duration={3}
				pathWidth={4}
				curvature={60}
				reverse
			/>
			<AnimatedBeam
				containerRef={containerRef}
				fromRef={datasetRef}
				toRef={nodeRef}
				className="-z-10"
				duration={3}
				pathWidth={4}
				curvature={60}
				reverse
			/>

			<div className="mt-14 flex justify-end gap-8 sm:-mt-44 sm:justify-start sm:pl-20 lg:mt-0 lg:pl-0">
				<div className="ml-auto w-44 flex-none space-y-8 pt-32 sm:ml-0 sm:pt-80 lg:order-last lg:pt-36 xl:order-none xl:pt-80">
					<PreviewCard
						ref={card1Ref}
						src="https://images.unsplash.com/photo-1557804506-669a67965ba0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
						dockingPointColor="#683DFD"
					/>
				</div>
				<div className="mr-auto w-44 flex-none space-y-8 sm:mr-0 sm:pt-64 lg:pt-36">
					<PreviewCard
						ref={card2Ref}
						src="https://images.unsplash.com/photo-1485217988980-11786ced9454?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
						dockingPointColor="#E337FF"
					/>
					<PreviewCard
						ref={card3Ref}
						src="https://images.unsplash.com/photo-1559136555-9303baea8ebd?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-x=.4&w=396&h=528&q=80"
						dockingPointColor="#01D3B1"
					/>
				</div>
				<div className="w-44 flex-none space-y-8 pt-32 sm:pt-48 lg:pt-0">
					<PreviewCard
						ref={card4Ref}
						src="https://images.unsplash.com/photo-1670272504528-790c24957dda?ixlib=rb-4.0.3&ixid=MnwxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=left&w=400&h=528&q=80"
						dockingPointColor="#FE3863"
					/>
					<PreviewCard
						ref={card5Ref}
						src="https://images.unsplash.com/photo-1670272505284-8faba1c31f7d?ixlib=rb-4.0.3&ixid=MnwxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
						dockingPointColor="#FCB600"
					/>
				</div>
			</div>

			<div className="mt-16 flex flex-col items-center justify-center">
				<Circle ref={nodeRef} className="h-16 w-16 self-start">
					<LogoIcon className="h-6 w-6" />
				</Circle>
				<div className="mt-8 w-full max-w-64 sm:max-w-[26rem]">
					<div className="flex flex-col items-center justify-center sm:flex-row sm:gap-4">
						<div className="relative w-full">
							<Select>
								<SelectTrigger className="w-full bg-white">
									<SelectValue placeholder="Select template" />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="apple">Apple</SelectItem>
									<SelectItem value="banana">Banana</SelectItem>
									<SelectItem value="blueberry">Blueberry</SelectItem>
									<SelectItem value="grapes">Grapes</SelectItem>
									<SelectItem value="pineapple">Pineapple</SelectItem>
								</SelectContent>
							</Select>
							<div className="absolute left-0 right-0 top-0 mx-auto h-2 w-2" ref={templateRef} />
						</div>
						<p className="w-8 text-2xl font-bold text-gray-600">+</p>
						<div className="relative w-full">
							<Select>
								<SelectTrigger className="w-full bg-white">
									<SelectValue placeholder="Select dataset" />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="apple">Apple</SelectItem>
									<SelectItem value="banana">Banana</SelectItem>
									<SelectItem value="blueberry">Blueberry</SelectItem>
									<SelectItem value="grapes">Grapes</SelectItem>
									<SelectItem value="pineapple">Pineapple</SelectItem>
								</SelectContent>
							</Select>
							<div
								className="absolute left-8 top-0 h-2 w-2 sm:left-0 sm:right-0 sm:mx-auto"
								ref={datasetRef}
							/>
						</div>
					</div>
					<p className="text-md mt-4 text-gray-600">
						To auto-generate designs, select a template and dataset.
					</p>
				</div>
			</div>
		</div>
	);
};
