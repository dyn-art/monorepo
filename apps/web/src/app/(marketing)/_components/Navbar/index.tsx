import { marketingConfig } from '@/environment';

import { DesktopNavContent } from './DesktopNavContent';
import { MobileNavContent } from './MobileNavContent';
import { TNavLink } from './types';

export const Navbar: React.FC = () => {
	return (
		<header className={'font-body sticky top-4 z-50 justify-center px-6 md:flex md:px-0'}>
			<DesktopNavContent
				links={marketingConfig.navbar.links as TNavLink[]}
				className={'hidden bg-[#FCFAF4]/70 md:flex'}
			/>
			<MobileNavContent
				links={marketingConfig.navbar.links as TNavLink[]}
				className={'bg-[#FCFAF4]/70 md:hidden'}
			/>
		</header>
	);
};
