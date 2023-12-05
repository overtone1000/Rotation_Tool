import { PointString } from './PointString';
import { RectBounds } from './RectBounds';

export const drawReactPolyline = function (
	ps: PointString,
	color: string,
	dashed: boolean = false,
	width: number = 1
) {
	let strokearray: string = '';
	if (dashed) {
		strokearray = (width * 2).toString() + ',' + (width * 3).toString();
	}
	/*
    return (
        <polyline
            key={key}
            points={ps.get()}
            fill="none"
            stroke={color}
            strokeWidth={width.toString()}
            strokeLinecap="square"
            strokeDasharray={strokearray}
            >

        </polyline>
    );
    */
};
export const drawXLabel = function (
	x: number,
	top: number,
	bottom: number,
	text: string,
	color: string,
	left_label: boolean = true
) {
	const retval = [];

	const label_angle = -Math.PI / 2.0;

	const degperrad = 180.0 / Math.PI;

	let ps = new PointString();
	ps.add(x, bottom);
	ps.add(x, top);

	//retval.push(drawReactPolyline(ps,color,false,1,"0"));

	let text_location_x = x;
	let text_location_y = (bottom + top) / 2.0;

	let baseline: string;
	if (left_label) {
		baseline = 'hanging';
	} else {
		text_location_x -= 2;
		baseline = 'text-bottom';
	}

	let transform =
		'rotate(' + label_angle * degperrad + ',' + text_location_x + ',' + text_location_y + ')';
	//console.log("transform = " + transform);

	//fill WAS theme.dark
	/*
    retval.push(
        <text
            fontFamily={autoscheda_fontFamily}
            x={text_location_x}
            y={text_location_y}
            transform={transform}
            textAnchor="middle"
            textLength={(bottom-top)*0.8}
            pointerEvents="none"
            lengthAdjust="spacingAndGlyphs"
            dominantBaseline={baseline}
            fill={color}
            key={key}
        >
            {text}
        </text>
    );
*/

	return retval;
};

export let drawText = function (
	bounds: RectBounds,
	color: string,
	text: string,
	text_anchor: string = 'middle'
) {
	//console.debug("Drawing text...")
	const rect = bounds.clone();

	let x: number;

	switch (text_anchor) {
		case 'start':
			x = rect.left + 1;
			break;
		case 'end':
			x = rect.right - 1;
			break;
		case 'middle':
		default:
			x = rect.horizontalMiddle();
			break;
	}

	let rect_height = rect.height();
	let rect_width = rect.width();
	let font_size: number;
	if (rect_height < 16) {
		font_size = rect_height;
	} else {
		font_size = 16;
	}

	let max_width = rect_width - 40;

	//if(textelement.getBBox().width>max_width)
	//{
	//	textelement.setAttribute("textLength",max_width);
	//	textelement.setAttribute("lengthAdjust","spacingAndGlyphs");
	//}

	console.debug('Text fill is', color);

	/*
	return (
        <text
            fontFamily={autoscheda_fontFamily}
            x={x}
            y={rect.verticalMiddle()}
            width={rect_width}
            height={rect_height}
            fontSize={font_size}
            //color={color}
            fill={color}
            textAnchor={text_anchor}
            dominantBaseline="middle"
            pointerEvents="none"
            lengthAdjust="spacingAndGlyphs"
            key={key}
            >
            {text}
        </text>
    );
    */
};

export const drawBox = function (
	bounds: RectBounds,
	fill: string,
	stroke?: string,
	strokewidth?: number,
	interaction_handler?: (evt: any) => void
) {
	/*
    return (
        <ASBox
            rect={bounds.clone()}
            fill={fill}
            key={"b" + key}
            stroke={stroke}
            strokewidth={strokewidth}
            interaction_handler={interaction_handler}
            >
        </ASBox>
    );
    */
};

export const drawTextBox = function (
	bounds: RectBounds,
	fill: string,
	text: string,
	textcolor: string,
	text_anchor?: string,
	stroke?: string,
	strokewidth?: number,
	interaction_handler?: (evt: any) => void
) {
	/*
    const box:React.ReactNode = drawBox(bounds,fill,key, stroke, strokewidth, interaction_handler);
    const textelement:React.ReactNode = drawText(bounds,textcolor,text,text_anchor,"t"+key);
    return [box,textelement];
    */
};
