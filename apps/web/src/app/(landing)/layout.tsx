import { Container } from '@dyn/ui';

const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return <Container tag="main">{children}</Container>;
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
