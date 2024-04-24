import React from 'react';
import type { TMdtifComposition } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';
import {
	Badge,
	Bird,
	Input,
	Label,
	Rabbit,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Skeleton,
	Textarea,
	Turtle,
	useSizeCallback
} from '@dyn/ui';
import { usePreparedDtif } from '@/hooks';

import { Viewport } from './components';

export const FieldBasedEditor: React.FC<TFieldBasedEditorProps> = (props) => {
	const { mdtif } = props;
	const [composition, setComposition] = React.useState<Composition | null>(null);
	const viewportRef = React.useRef<HTMLDivElement>(null);
	const { data: preparedDtif, isLoading: isPreparingDtif } = usePreparedDtif(mdtif?.template);

	useSizeCallback(
		viewportRef,
		// Not passing the viewport size as prop to the Canvas or in the DTIF
		// because React is kinda slow updating their states
		(size) => {
			composition?.emitInputEvents('Composition', [
				{
					type: 'CompositionResized',
					size: [size.width, size.height]
				},
				{ type: 'FocusRootNodes' }
			]);
			composition?.update();
		},
		[composition]
	);

	return (
		<div className="grid flex-1 gap-4 overflow-auto p-4 md:grid-cols-2 lg:grid-cols-3">
			<form className="grid w-full items-start gap-6">
				<fieldset className="grid gap-6 rounded-lg border p-4">
					<legend className="-ml-1 px-1 text-sm font-medium">Settings</legend>
					<div className="grid gap-3">
						<Label htmlFor="model">Model</Label>
						<Select>
							<SelectTrigger className="items-start [&_[data-description]]:hidden" id="model">
								<SelectValue placeholder="Select a model" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="genesis">
									<div className="text-muted-foreground flex items-start gap-3">
										<Rabbit className="size-5" />
										<div className="grid gap-0.5">
											<p>
												Neural <span className="text-foreground font-medium">Genesis</span>
											</p>
											<p className="text-xs" data-description>
												Our fastest model for general use cases.
											</p>
										</div>
									</div>
								</SelectItem>
								<SelectItem value="explorer">
									<div className="text-muted-foreground flex items-start gap-3">
										<Bird className="size-5" />
										<div className="grid gap-0.5">
											<p>
												Neural <span className="text-foreground font-medium">Explorer</span>
											</p>
											<p className="text-xs" data-description>
												Performance and speed for efficiency.
											</p>
										</div>
									</div>
								</SelectItem>
								<SelectItem value="quantum">
									<div className="text-muted-foreground flex items-start gap-3">
										<Turtle className="size-5" />
										<div className="grid gap-0.5">
											<p>
												Neural <span className="text-foreground font-medium">Quantum</span>
											</p>
											<p className="text-xs" data-description>
												The most powerful model for complex computations.
											</p>
										</div>
									</div>
								</SelectItem>
							</SelectContent>
						</Select>
					</div>
					<div className="grid gap-3">
						<Label htmlFor="temperature">Temperature</Label>
						<Input id="temperature" placeholder="0.4" type="number" />
					</div>
					<div className="grid grid-cols-2 gap-4">
						<div className="grid gap-3">
							<Label htmlFor="top-p">Top P</Label>
							<Input id="top-p" placeholder="0.7" type="number" />
						</div>
						<div className="grid gap-3">
							<Label htmlFor="top-k">Top K</Label>
							<Input id="top-k" placeholder="0.0" type="number" />
						</div>
					</div>
				</fieldset>
				<fieldset className="grid gap-6 rounded-lg border p-4">
					<legend className="-ml-1 px-1 text-sm font-medium">Messages</legend>
					<div className="grid gap-3">
						<Label htmlFor="role">Role</Label>
						<Select defaultValue="system">
							<SelectTrigger>
								<SelectValue placeholder="Select a role" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="system">System</SelectItem>
								<SelectItem value="user">User</SelectItem>
								<SelectItem value="assistant">Assistant</SelectItem>
							</SelectContent>
						</Select>
					</div>
					<div className="grid gap-3">
						<Label htmlFor="content">Content</Label>
						<Textarea className="min-h-[9.5rem]" id="content" placeholder="You are a..." />
					</div>
				</fieldset>
			</form>
			<div className="bg-muted/50 relative flex h-full min-h-[50vh] flex-col overflow-hidden rounded-xl lg:col-span-2">
				{isPreparingDtif || preparedDtif == null ? (
					<Skeleton className="h-full w-full rounded-none" />
				) : (
					<Viewport
						dtif={preparedDtif}
						onLoadedComposition={setComposition}
						viewportRef={viewportRef}
					/>
				)}
				<Badge className="absolute right-3 top-3 bg-white" variant="outline">
					Preview
				</Badge>
			</div>
		</div>
	);
};

export interface TFieldBasedEditorProps {
	mdtif?: TMdtifComposition;
}
