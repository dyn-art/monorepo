import React from 'react';
import type { FigmaAppHandler, TAppCallbackRegistration } from '@dyn/figma-handler/app';

export function useAppCallback<GFigmaAppHandler extends FigmaAppHandler>(
	appHandler: GFigmaAppHandler,
	registrations:
		| TAppCallbackRegistration<ExtractPluginMessageEvent<GFigmaAppHandler>>
		| TAppCallbackRegistration<ExtractPluginMessageEvent<GFigmaAppHandler>>[]
): void {
	React.useEffect(() => {
		appHandler.register(registrations);
	}, [registrations]);
}

type ExtractPluginMessageEvent<T> = T extends FigmaAppHandler<infer U> ? U : never;
