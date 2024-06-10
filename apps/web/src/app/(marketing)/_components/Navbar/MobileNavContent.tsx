'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import React from 'react';
import {
	Button,
	cn,
	HamburgerMenuIcon,
	LogoIcon,
	Popover,
	PopoverContent,
	PopoverTrigger
} from '@dyn/ui';

import { TNavLink } from './types';

export const MobileNavContent: React.FC<TProps> = (props) => {
	const { links, className } = props;
	const pathname = usePathname();
	const lastPath = React.useMemo(() => `/${pathname.split('/').pop()}`, [pathname]);

	return (
		<div
			className={cn(
				'border-border flex h-[50px] items-center rounded-md border px-4 backdrop-blur-xl backdrop-filter',
				className
			)}
		>
			<Link href="/">
				<span className="sr-only">dyn.art Logo</span>
				<LogoIcon className="h-6 w-6" />
			</Link>

			<Button asChild variant={'ghost'} className="ml-auto px-3 py-2 font-medium">
				<Link href={'https://dyn.art'}>Join Waitlist</Link>
			</Button>

			<div className="border-border mx-4 h-6 border-l-[1px]" />

			<Popover>
				<PopoverTrigger asChild>
					<Button variant={'ghost'} size={'icon'}>
						<HamburgerMenuIcon className="h-5 w-5" />
					</Button>
				</PopoverTrigger>
				<PopoverContent align="start" className="mr-2 mt-2 bg-[#FCFAF4]">
					<nav>
						<ul className="mb-6 space-y-6 px-3 pt-6 text-xl">
							{links.map(({ path, title }) => {
								const isActive = path === lastPath;

								return (
									<li key={path}>
										<Link href={path} className={cn(isActive && 'underline')}>
											{title}
										</Link>
									</li>
								);
							})}

							<li className="mt-auto border-t-[1px] pt-8">
								<Link className="text-primary text-xl" href="https://dyn.art/app">
									Join Waitlist
								</Link>
							</li>
						</ul>
					</nav>
				</PopoverContent>
			</Popover>
		</div>
	);
};

interface TProps {
	links: TNavLink[];
	className?: string;
}
