import type { ASDisplayResponseData } from '../../ancillary/ajax/commands_generic';
import type { DisplaySync } from '../displays/DisplaySync';
import type { Instantiator } from '../input/preinstantiation';

export enum DataType {
	Boolean,
	Integer,
	Long,
	Float,
	DisableableDouble,
	LocalDate,
	LocalDateAsEpochDay,
	LocalTime,
	String,
	Binary,
	Array,
	Enum,
	EnumArray,
	NodeReference,
	TypedDetailsField,
	DynamicOptionList
}

export interface Labels {
	map: { [i: number | string]: string };
	order: number[];
	disabled?: (number | string)[];
}

export interface DataMeta {
	nullable?: boolean;
	exclusive?: boolean;
	instantiators: any;
	filterable: boolean;
	labels: Labels;
	translators: { [i: number]: { [i: number | string]: string } };
	translator_orders: { [i: number]: number[] };
	retireable: boolean | undefined;
	i: { [i: number]: Instantiator } | undefined;
	assignment_types: ASDisplayResponseData | undefined;
	schedule_template_types: ASDisplayResponseData | undefined;
}

export interface DisplayNode {
	element: any;
	style?: any;
}

export function toDisplayNode(
	update_data: ASDisplayResponseData,
	colkey: string,
	rowkey: string,
	syncer: DisplaySync
): DisplayNode {
	//type:DataType, meta:DataMeta, val:any,
	const type: DataType = update_data.cols[colkey].data_type;
	const val = update_data.rows[rowkey][colkey];
	const colmeta = update_data.cols[colkey].meta;
	switch (type) {
		case DataType.Boolean:
			/*
            if(val===undefined)
            {
                return {element:(<QuestionMarkIcon/>)};
            }
            else if(val===null)
            {
                return {element:null};
            }
            else if(val)
            {
                return {element:(<CheckIcon/>)};
            }
            else
            {
                return {element:(<Block/>)};
            }
            */
			return {} as DisplayNode; //TODO
		case DataType.Integer:
		case DataType.Float:
		case DataType.LocalDate:
		case DataType.LocalTime:
		case DataType.String:
			//return {element:<Typography>{val}</Typography>};
			return {} as DisplayNode; //TODO
		case DataType.DisableableDouble: {
			/*
                if(val)
                {
                    return {element:<Typography>{val}</Typography>};
                }
                else
                {
                    return {element:<BlockIcon/>};
                }
                */
			return {} as DisplayNode; //TODO
		}
		case DataType.Binary: {
			/*
                const node:BinaryNode = new BinaryNode(update_data.meta, update_data.cols[colkey].meta, val as BNData);
                const result:BinaryNodeDisplay = node.createDisplayNode(syncer);

                let segment_display=null;
                if(result.custom_display!==undefined && result.custom_display!==null)
                {
                    console.debug("Segment display isn't undefined!");
                    segment_display=
                    (
                        <Box flexGrow={1} overflow="hidden">
                            {result.custom_display}
                        </Box>
                    );
                }

                return {element:(
                    <Box display="flex" flexDirection="row" width="100%" overflow="hidden">
                        {segment_display}
                        <Box flexShrink={1} overflow="hidden">
                            {result.child}
                        </Box>
                    </Box>
                    ),
                    style:result.custom_style
                };
                */
			return {} as DisplayNode; //TODO
		}
		case DataType.Enum:
			/*
            return {element:<Typography>{colmeta.labels.map[val]}</Typography>};
        case DataType.EnumArray:
            let retval="";
            for(const key in val)
            {
                retval += colmeta.labels.map[val[key]] + ", ";
            }
            retval=retval.substring(0,retval.length-2);
            return {element:<Typography>{retval}</Typography>};
            */
			return {} as DisplayNode; //TODO
		default:
			console.error('Unhandled type ' + type + ' with value ' + val);
			//return {element:<Typography>{"Unhandled type " + type + " with value " + val}</Typography>};
			return {} as DisplayNode; //TODO
	}
}
