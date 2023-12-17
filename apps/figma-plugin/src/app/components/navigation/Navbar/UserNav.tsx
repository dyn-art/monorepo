import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import {
	Avatar,
	AvatarFallback,
	AvatarImage,
	Button,
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuGroup,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
	ExternalLinkIcon
} from '@dyn/ui';

import { EAppRoutes } from '../../../../types';

export const UserNav: React.FC = () => {
	const navigate = useNavigate();

	return (
		<DropdownMenu>
			<DropdownMenuTrigger asChild>
				<Button variant="ghost" className="relative h-8 w-8 rounded-full">
					<Avatar className="h-8 w-8">
						<AvatarImage src="https://github.com/bennoinbeta.png" alt="@bennoinbeta" />
						<AvatarFallback>SC</AvatarFallback>
					</Avatar>
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent className="w-56" align="end" forceMount>
				<DropdownMenuLabel className="font-normal">
					<div className="flex flex-col space-y-1">
						<p className="text-sm font-medium leading-none">bennoinbeta</p>
						<p className="text-muted-foreground text-xs leading-none">m@bennoinbeta.com</p>
					</div>
				</DropdownMenuLabel>
				<DropdownMenuSeparator />
				<DropdownMenuGroup>
					<DropdownMenuItem className="cursor-pointer" asChild>
						<div className="flex flex-row justify-between">
							<Link target="_blank" to={'https://app.dyn.art/profile/figma?source=figma'}>
								Profile
							</Link>
							<ExternalLinkIcon />
						</div>
					</DropdownMenuItem>
					<DropdownMenuItem className="cursor-pointer" asChild>
						<Link to={EAppRoutes.SETTINGS}>Settings</Link>
					</DropdownMenuItem>
				</DropdownMenuGroup>
				<DropdownMenuSeparator />
				<DropdownMenuItem>Log out</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
};
