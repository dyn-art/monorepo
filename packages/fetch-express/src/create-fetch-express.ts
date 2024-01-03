import { FetchExpress } from './FetchExpress';

export function createFetchExpress<GPaths extends {}>(): FetchExpress<GPaths> {
	return new FetchExpress<GPaths>();
}
