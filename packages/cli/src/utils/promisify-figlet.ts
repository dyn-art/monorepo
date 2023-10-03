import figlet from 'figlet';

export function promisifyFiglet(text: string): Promise<string> {
	return new Promise((resolve, reject) => {
		figlet(text, (err, data) => {
			if (err || data == null) {
				reject(err);
			} else {
				resolve(data);
			}
		});
	});
}
