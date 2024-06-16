'use client';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import React from 'react';
import { ButtonProps, composeRefs, getElementRef, hasDisplayName, Slot } from '@dyn/ui';

export const LoginSlot = React.forwardRef<HTMLElement, TProps>((props, forwardRef) => {
	const { children, mode = 'redirect' } = props;
	const router = useRouter();

	const onClick = React.useCallback(() => {
		router.push('/auth/login');
	}, [router]);

	if (!React.isValidElement(children)) {
		return null;
	}

	// If child component is our Button component than make it a Link Button
	if (mode === 'redirect' && hasDisplayName(children, 'Button')) {
		const childrenRef = getElementRef(children);
		return React.cloneElement(children, {
			...children.props,
			asChild: true,
			children: <Link href="/auth/login">{children.props.children}</Link>,
			ref: composeRefs(forwardRef, childrenRef)
		} as ButtonProps);
	}

	return (
		<Slot ref={forwardRef} onClick={onClick}>
			{children}
		</Slot>
	);
});

interface TProps {
	children: React.ReactNode;
	mode?: 'modal' | 'redirect';
}
