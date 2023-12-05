import { CursorRelevantState, setCursorRelevantState } from '../commons/DOMfunctions';
import { RectBounds } from './RectBounds';

interface ASBox_props {
	rect: RectBounds;
	fill: string;
	stroke?: string;
	strokewidth?: number;
	interaction_handler?: (evt: any) => void;
}
interface ASBox_state {}
export class ASBox extends React.Component<ASBox_props, ASBox_state> {
	constructor(props: ASBox_props) {
		super(props);
		this.state = { outline_color: null };
	}

	render() {
		const width = this.props.rect.width();
		const height = this.props.rect.height();
		if (width <= 0 || height <= 0) {
			return <div />;
		}

		let stroke: string;
		if (this.props.stroke !== undefined) {
			stroke = this.props.stroke;
		} else {
			stroke = null;
		}
		let strokewidth: number;
		if (this.props.strokewidth !== undefined) {
			strokewidth = this.props.strokewidth;
		} else {
			strokewidth = 0;
		}

		/*
            const sx:React.CSSProperties = {};
    sx["&:hover"]={
      cursor: "pointer"
    };
*/

		let onClick = null;
		let onMouseEnter = null;
		let onMouseLeave = null;
		if (this.props.interaction_handler !== undefined && this.props.interaction_handler !== null) {
			onClick = (evt: any) => {
				this.props.interaction_handler(evt);
			};
			onMouseEnter = (evt: any) => {
				setCursorRelevantState(CursorRelevantState.HoverOverInteractable, true);
				this.props.interaction_handler(evt);
			};
			onMouseLeave = (evt: any) => {
				setCursorRelevantState(CursorRelevantState.HoverOverInteractable, false);
				this.props.interaction_handler(evt);
			};
		}

		return (
			<rect
				x={this.props.rect.left}
				y={this.props.rect.top}
				width={width}
				height={height}
				stroke={stroke}
				strokeWidth={strokewidth}
				fill={this.props.fill}
				onClick={onClick}
				onMouseEnter={onMouseEnter}
				onMouseLeave={onMouseLeave}
			></rect>
		);
	}
}
