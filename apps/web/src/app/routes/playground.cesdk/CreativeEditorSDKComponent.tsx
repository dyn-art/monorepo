import type { _RequiredCreateConfiguratioan } from '@cesdk/cesdk-js';
import React from 'react';

const CreativeEditorSDKComponent: React.FC<TProps> = (props) => {
	const { creativeEditorConfig = {} } = props;

	const cesdk_container = React.useRef<HTMLDivElement | null>(null);

	React.useEffect(() => {
		if (cesdk_container.current != null) {
			(async () => {
				const CreativeEditorSDK = (await import('@cesdk/cesdk-js')).default;

				// Enable local uploads in Asset Library
				creativeEditorConfig.callbacks = { onUpload: 'local' };

				CreativeEditorSDK.create(
					cesdk_container.current as HTMLDivElement,
					creativeEditorConfig
				).then(async (instance) => {
					// Do something with the instance of CreativeEditor SDK, for example:
					// Populate the asset library with default / demo asset sources.
					await Promise.all([
						instance.addDefaultAssetSources(),
						instance.addDemoAssetSources({ sceneMode: 'Design' })
					]);
					await instance.createDesignScene();
				});
			})();
		}
	}, [creativeEditorConfig, cesdk_container]);

	return <div ref={cesdk_container} style={{ width: '100vw', height: '100vh' }}></div>;
};

export { CreativeEditorSDKComponent };

type TProps = {
	creativeEditorConfig?: Partial<_RequiredCreateConfiguratioan>;
};
