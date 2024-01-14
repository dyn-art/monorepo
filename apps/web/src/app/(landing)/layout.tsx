const Layout: React.FC<TProps> = (props) => {
	const { children } = props;

	return <>{children}</>;
};

export default Layout;

type TProps = { children: React.ReactNode };
