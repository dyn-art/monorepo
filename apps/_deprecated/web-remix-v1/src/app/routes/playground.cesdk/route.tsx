import React from 'react';

let isHydrating = true;

const CeSDK: React.FC = () => {
	const [CreativeEditorSDK, setCreativeEditorSDK] = React.useState<React.ElementType | null>(null);
	const [isHydrated, setIsHydrated] = React.useState(!isHydrating);

	React.useEffect(() => {
		isHydrating = false;
		setIsHydrated(true);

		if (typeof window !== 'undefined') {
			import('./CreativeEditorSDKComponent')
				.then((module) => {
					setCreativeEditorSDK(() => module.CreativeEditorSDKComponent);
				})
				.catch((error) => {
					console.error('Error loading the CreativeEditorSDKComponent:', error);
				});
		}
	}, []);

	if (isHydrated && CreativeEditorSDK) {
		return <CreativeEditorSDK />;
	}
	return null;
};

export default CeSDK;
