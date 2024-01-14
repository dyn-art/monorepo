const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return <>{children}</>;
};

export default Layout;

interface TProps {
	children: React.ReactNode;
}
