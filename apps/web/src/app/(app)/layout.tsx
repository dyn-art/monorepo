import { LayoutWrapper } from '@dyn/ui';

const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return (
		<LayoutWrapper size="default" asChild>
			<main>{children}</main>
		</LayoutWrapper>
	);
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
