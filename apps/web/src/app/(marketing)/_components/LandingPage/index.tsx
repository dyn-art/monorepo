'use client';

import React from 'react';
import { AdvancedInput, Button, ScribbleRepeatIcon } from '@dyn/ui';

import { triggerFirework } from './controller';
import { Preview } from './Preview';

export const LandingPage: React.FC = () => {
	return (
		<div className="mx-auto max-w-screen-xl px-6 pb-32 pt-16 sm:pt-32 lg:px-8">
			<div className="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
				<div className="relative w-full max-w-xl lg:shrink-0">
					<h1 className="font-display max-w-[20rem] text-[4rem] font-black leading-[6rem] sm:max-w-[28rem] sm:text-[6rem] sm:leading-[7rem]">
						Leave the <mark className="bg-transparent text-[#683DFD]">repetitive design</mark> to us
					</h1>
					<ScribbleRepeatIcon className="absolute left-64 top-28 h-16 w-16 rotate-45 text-[#683DFD] sm:left-96 sm:top-32 sm:h-20 sm:w-20" />

					<p className="font-body mt-8 text-xl sm:text-2xl">
						Your tool for <mark className="bg-transparent font-bold">auto-generating</mark> posters,
						social media visuals, and more through our user-friendly{' '}
						<mark className="bg-transparent font-bold">API</mark> and{' '}
						<mark className="bg-transparent font-bold">integrations</mark>
					</p>

					<div className="mt-12 max-w-[28rem]">
						<AdvancedInput
							placeholder={'Enter your Email'}
							className="pr-40"
							size={'xl'}
							childrenAfter={
								<div className="absolute inset-y-0 right-0 flex items-center pr-3">
									<Button size={'lg'} onClick={triggerFirework}>
										Join Waitlist
									</Button>
								</div>
							}
						/>
					</div>
				</div>
				<Preview />
			</div>

			{/* <p>TODO</p> */}
		</div>
	);
};
