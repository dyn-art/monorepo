import { Container } from '@dyn/ui';

const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return (
		<Container size="compact" tag="main">
			{children}
		</Container>
	);
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
