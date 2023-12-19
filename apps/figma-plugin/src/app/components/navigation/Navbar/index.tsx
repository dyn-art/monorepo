import React, { type ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { Button, ChevronLeftIcon, InfoCircledIcon, Separator } from '@dyn/ui';

import { UserNav } from './UserNav';

export const Navbar: React.FC<TProps> = (props) => {
	const { leftContent, centerText, rightContent } = props;

	return (
		<>
			<div className={`flex h-8 w-full items-center justify-between px-4`}>
				{leftContent != null && (
					<div className="flex text-left">{renderLeftContent(leftContent)}</div>
				)}
				{centerText != null && (
					<div className="pointer-events-none absolute left-0 right-0 ml-auto mr-auto px-4 text-center">
						<p>{centerText}</p>
					</div>
				)}
				{rightContent != null && (
					<div className="flex text-right">{renderRightContent(rightContent)}</div>
				)}
			</div>
			<Separator className="mt-2" />
		</>
	);
};

const renderLeftContent = (leftContent: TLeftContent) => {
	if (typeof leftContent !== 'object' || leftContent == null || !('variant' in leftContent)) {
		return leftContent;
	}

	switch (leftContent.variant) {
		case 'back':
			return (
				<Button variant="ghost" size="icon" onClick={leftContent.onClick}>
					<ChevronLeftIcon className="h-4 w-4" />
				</Button>
			);
		default:
			return null;
	}
};

const renderRightContent = (rightContent: TRightContent) => {
	if (typeof rightContent !== 'object' || rightContent == null || !('variant' in rightContent)) {
		return rightContent;
	}

	switch (rightContent.variant) {
		case 'user':
			return <UserNav />;
		case 'info':
			return (
				<Button variant="ghost" size="icon" asChild>
					<Link target="_blank" to={'https://dyn.art/figma?source=figma'}>
						<InfoCircledIcon className="h-4 w-4" />
					</Link>
				</Button>
			);
		default:
			return null;
	}
};

interface TProps {
	leftContent?: TLeftContent;
	centerText?: string;
	rightContent?: TRightContent;
}

type TLeftContent = ReactNode | TLeftContentVariants;
type TLeftContentVariants = TLeftContentBack;
interface TLeftContentBack {
	variant: 'back';
	onClick: () => void;
}

type TRightContent = ReactNode | TRightContentVariants;
type TRightContentVariants = TRightContentUser | TRightContentInfo;
interface TRightContentUser {
	variant: 'user';
}
interface TRightContentInfo {
	variant: 'info';
	url: string;
}
