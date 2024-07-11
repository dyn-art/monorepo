'use client';

import React from 'react';
import { AdvancedInput, Button, ScribbleRepeatIcon } from '@dyn/ui';

import { triggerFirework } from './controller';
import { FeatureLane } from './FeatureLane';
import { Preview } from './Preview';

export const LandingPage: React.FC = () => {
	return (
		<>
			<div className="mx-auto max-w-screen-xl px-6 pb-32 pt-32 lg:px-8 lg:pt-0">
				<div className="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
					<div className="relative w-full max-w-xl lg:shrink-0">
						<h1 className="font-display max-w-[20rem] text-[4rem] font-black leading-[6rem] sm:max-w-[28rem] sm:text-[6rem] sm:leading-[7rem]">
							Leave the <mark className="bg-transparent text-[#683DFD]">repetitive design</mark> to
							us
						</h1>
						<ScribbleRepeatIcon className="absolute left-64 top-28 h-16 w-16 rotate-45 text-[#683DFD] sm:left-96 sm:top-32 sm:h-20 sm:w-20" />

						<p className="font-body mt-8 text-xl sm:text-2xl">
							Your tool for <mark className="bg-transparent font-bold">auto-generating</mark>{' '}
							posters, social media visuals, and more through our user-friendly{' '}
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
			</div>

			<FeatureLane
				items={[
					{ type: 'Icon', name: 'Photoshop Integration (soon)', icon: '🖌️' },
					{ type: 'Icon', name: 'Inhouse Design Editor', icon: '🛠️' },
					{ type: 'Text', text: 'DESIGN' },
					{ type: 'Icon', name: 'Figma Integration', icon: '🔌' },
					{ type: 'Text', text: 'DESIGN' }
				]}
				backgroundColor="#FE3863"
				rotationInDeg={1}
				duration={60}
				leftToRight={false}
			/>

			<FeatureLane
				items={[
					{ type: 'Icon', name: 'Zapier Integration (soon)', icon: '🔗' },
					{ type: 'Icon', name: 'REST API', icon: '🌐' },
					{ type: 'Text', text: 'AUTOMATE' },
					{ type: 'Icon', name: 'Airtable Integration (soon)', icon: '📊' },
					{ type: 'Icon', name: 'Etsy Integration (soon)', icon: '🛍️' },
					{ type: 'Text', text: 'AUTOMATE' }
				]}
				backgroundColor="#01D3B1"
				rotationInDeg={-0.5}
				duration={60}
				leftToRight={true}
				className="mt-10"
			/>

			<FeatureLane
				items={[
					{ type: 'Icon', name: 'Pay what you use', icon: '💳' },
					{ type: 'Icon', name: 'Self Hosting', icon: '🏠' },
					{ type: 'Text', text: 'SCALE' },
					{ type: 'Icon', name: 'Custom Buckets', icon: '🗂️' },
					{ type: 'Text', text: 'SCALE' }
				]}
				backgroundColor="#FCB600"
				rotationInDeg={0.5}
				duration={60}
				leftToRight={false}
				className="mt-8"
			/>

			<div className="mx-auto max-w-screen-xl px-6 pt-32 lg:px-8">{/* <p>todo</p> */}</div>
		</>
	);
};
