import type { FigmaAppHandler, TAppCallbackRegistration } from 'figma-connect/app';
import React from 'react';

export function useAppCallback<GFigmaAppHandler extends FigmaAppHandler>(
	appHandler: GFigmaAppHandler,
	registrations:
		| TAppCallbackRegistration<ExtractPluginMessageEvent<GFigmaAppHandler>>
		| TAppCallbackRegistration<ExtractPluginMessageEvent<GFigmaAppHandler>>[],
	deps: React.DependencyList = []
): void {
	React.useEffect(() => {
		const unregisterFunctions = appHandler.register(registrations);
		return () => {
			unregisterFunctions.forEach((unregisterFunction) => {
				unregisterFunction();
			});
		};
	}, deps); // Note: Not registering "registrations" as dep as its an inline object whose pointer addr changes each render cycle
}

type ExtractPluginMessageEvent<T> = T extends FigmaAppHandler<infer U> ? U : never;
