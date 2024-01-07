import { createClient, FetchTransport } from '@rspc/client';
import type { Procedures } from '@dyn/types/comprender';

export const compRenderClient = createClient<Procedures>({
	transport: new FetchTransport('http://localhost:4000/rspc')
});
