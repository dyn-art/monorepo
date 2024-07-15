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
import { LoginSlot } from '@/components';

import { type TNavLink } from './types';

export const MobileNavContent: React.FC<TProps> = (props) => {
	const { links, className } = props;
	const pathname = usePathname();
	const lastPath = React.useMemo(() => `/${pathname.split('/').pop() ?? ''}`, [pathname]);

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

			<Button asChild variant="ghost" className="ml-auto px-3 py-2 font-medium">
				<Link href="https://dyn.art">Join Waitlist</Link>
			</Button>

			<div className="border-border mx-4 h-6 border-l-[1px]" />

			<Popover>
				<PopoverTrigger asChild>
					<Button variant="ghost" size="icon">
						<HamburgerMenuIcon className="h-5 w-5" />
					</Button>
				</PopoverTrigger>
				<PopoverContent align="start" className="mr-2 mt-2 bg-[#FCFAF4]">
					<nav>
						<ul className="mb-6 space-y-6 pt-6">
							{links.map(({ path, title }) => {
								const isActive = path === lastPath;

								return (
									<li key={path}>
										<Button
											className={cn('text-xl', isActive && 'underline')}
											asChild
											variant="ghost"
										>
											<Link href={path}>{title}</Link>
										</Button>
									</li>
								);
							})}

							<li className="border-t-[1px] pt-8">
								<LoginSlot>
									<Button variant="ghost" className="text-xl">
										Sign in
									</Button>
								</LoginSlot>
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
