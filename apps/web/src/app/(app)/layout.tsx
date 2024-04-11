import { Container } from '@dyn/ui';

const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	// TODO: h-screen should be set on body?
	return (
		<Container className="h-screen" size="full" tag="main">
			{children as any}
		</Container>
	);
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
