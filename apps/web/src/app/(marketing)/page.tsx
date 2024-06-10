import { AdvancedInput, Button, ScribbleRepeatIcon } from '@dyn/ui';

const Page: React.FC = () => {
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
									<Button size={'lg'}>Join Waitlist</Button>
								</div>
							}
						/>
					</div>
				</div>

				<div className="mt-14 flex justify-end gap-8 sm:-mt-44 sm:justify-start sm:pl-20 lg:mt-0 lg:pl-0">
					<div className="ml-auto w-44 flex-none space-y-8 pt-32 sm:ml-0 sm:pt-80 lg:order-last lg:pt-36 xl:order-none xl:pt-80">
						<div className="relative">
							<img
								src="https://images.unsplash.com/photo-1557804506-669a67965ba0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
								alt=""
								className="aspect-[2/3] w-full rounded-xl bg-gray-900/5 object-cover shadow-lg"
							/>
							<div className="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-inset ring-gray-900/10" />
						</div>
					</div>
					<div className="mr-auto w-44 flex-none space-y-8 sm:mr-0 sm:pt-64 lg:pt-36">
						<div className="relative">
							<img
								src="https://images.unsplash.com/photo-1485217988980-11786ced9454?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
								alt=""
								className="aspect-[2/3] w-full rounded-xl bg-gray-900/5 object-cover shadow-lg"
							/>
							<div className="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-inset ring-gray-900/10" />
						</div>
						<div className="relative">
							<img
								src="https://images.unsplash.com/photo-1559136555-9303baea8ebd?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-x=.4&w=396&h=528&q=80"
								alt=""
								className="aspect-[2/3] w-full rounded-xl bg-gray-900/5 object-cover shadow-lg"
							/>
							<div className="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-inset ring-gray-900/10" />
						</div>
					</div>
					<div className="w-44 flex-none space-y-8 pt-32 sm:pt-48 lg:pt-0">
						<div className="relative">
							<img
								src="https://images.unsplash.com/photo-1670272504528-790c24957dda?ixlib=rb-4.0.3&ixid=MnwxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=left&w=400&h=528&q=80"
								alt=""
								className="aspect-[2/3] w-full rounded-xl bg-gray-900/5 object-cover shadow-lg"
							/>
							<div className="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-inset ring-gray-900/10" />
						</div>
						<div className="relative">
							<img
								src="https://images.unsplash.com/photo-1670272505284-8faba1c31f7d?ixlib=rb-4.0.3&ixid=MnwxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
								alt=""
								className="aspect-[2/3] w-full rounded-xl bg-gray-900/5 object-cover shadow-lg"
							/>
							<div className="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-inset ring-gray-900/10" />
						</div>
					</div>
				</div>
			</div>
		</div>
	);
};

export default Page;
