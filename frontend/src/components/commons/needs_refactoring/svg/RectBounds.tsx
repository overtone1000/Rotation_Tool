export class RectBounds {
	left: number;
	top: number;
	right: number;
	bottom: number;
	constructor(left: number = 0, top: number = 0, right: number = 0, bottom: number = 0) {
		this.left = left;
		this.top = top;
		this.right = right;
		this.bottom = bottom;
	}
	width(): number {
		return this.right - this.left;
	}
	height(): number {
		return this.bottom - this.top;
	}
	horizontalMiddle(): number {
		return (this.right + this.left) / 2.0;
	}
	verticalMiddle(): number {
		return (this.bottom + this.top) / 2.0;
	}
	clone(): RectBounds {
		return new RectBounds(this.left, this.top, this.right, this.bottom);
	}
}
