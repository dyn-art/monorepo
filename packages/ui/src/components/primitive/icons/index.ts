export * from './AppleIcon';
export * from './GithubIcon';
export * from './GoogleIcon';
export * from './LogoIcon';
export * from './PaypalIcon';
export * from './SpinnerIcon';
export * from './TwitterIcon';

export {
	CircleIcon,
	CursorArrowIcon,
	SquareIcon,
	StarIcon,
	VercelLogoIcon
} from '@radix-ui/react-icons';
export {} from 'lucide-react';

// TODO: Not doing it like that (altough it would be neat)
// because it won't be tree shakable
// export const Icons = {
// 	Apple: AppleIcon
// };

// Apparently also not tree shakable
// https://github.com/evanw/esbuild/issues/1420
// export * as RadixIcon from '@radix-ui/react-icons';
// export * as LucideIcon from 'lucide-react';
