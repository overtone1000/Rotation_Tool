import { RectBounds } from './RectBounds';

export class PointString {
	pointstring: string;
	construct() {
		this.clear();
	}

	add(x: number, y: number) {
		if (this.pointstring === undefined) {
			this.clear();
		}
		this.pointstring += x.toString() + ',' + y.toString() + ' ';
	}

	makeBox(coords: RectBounds) {
		this.add(coords.left, coords.top);
		this.add(coords.right, coords.top);
		this.add(coords.right, coords.bottom);
		this.add(coords.left, coords.bottom);
		this.add(coords.left, coords.top);
	}

	get() {
		return this.pointstring;
	}

	clear() {
		this.pointstring = '';
	}
}
