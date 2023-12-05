export class Debouncer {
	last_call: number = 0;
	delay_ms: number;
	f: (...args) => void;

	constructor(delay_ms: number, f: (...args) => void) {
		this.f = f;
		this.delay_ms = delay_ms;
	}

	call(...args) {
		const token = ++this.last_call;
		new Promise<boolean>((resolve, reject) => {
			setTimeout(() => {
				if (this.last_call == token) {
					console.debug('Calling debounce function.');
					this.f(...args);
					resolve(true);
				} else {
					resolve(false);
				}
			}, this.delay_ms);
		});
	}
}
