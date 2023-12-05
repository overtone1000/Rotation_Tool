import { DataType, type DataMeta } from '../../needs_refactoring/data_processing/data_types';
import type { DisplaySync } from '../../needs_refactoring/displays/DisplaySync';
import { epochDayToLocalDate, shortDateString } from '../staging/data_processing/processing01';

export interface BNData {
	l: number; //label_index
	v?: string | number; //value
	t?: number; //translator key
	c?: BNData[]; //children
	//col_key:number,
	d: DataType; //data_type_enum
	i?: string; //instantiator key
	r: boolean; //readonly
	h: boolean; //hidden
}

export const deepBNDataClone = (source: BNData) => {
	const retval = {} as BNData;
	Object.assign(retval, source);
	if (source.c !== undefined) {
		retval.c = [] as BNData[];
		for (const source_child of source.c) {
			retval.c.push(deepBNDataClone(source_child));
		}
	}
	return retval;
};

export interface BinaryNodeDisplay {
	children: any[];
	custom_display: any | null;
}

export class BinaryNode {
	table_meta: DataMeta;
	column_meta: DataMeta;
	node_data: BNData;
	parent_node: BinaryNode | undefined = undefined;
	children: { [key: string]: BinaryNode } | null;
	local_key: number | undefined;
	handlers = {} as {
		segmentSelectHandler: undefined | ((assignment_key: string | null) => void);
		//assignmentSelectHandler:undefined|((state:any)=>void),
		//constraintSelectHandler:undefined|((state:any)=>void)
	};

	constructor(
		table_meta: DataMeta,
		column_meta: DataMeta,
		node_data: BNData,
		local_key?: number,
		parent_node?: BinaryNode
	) {
		//console.debug("Constructing Binary Node",node_data);

		this.table_meta = table_meta;
		this.column_meta = column_meta;
		this.node_data = node_data;
		this.local_key = local_key;
		if (parent_node !== undefined) {
			this.parent_node = parent_node;
		}

		const new_children = {} as { [key: number]: BinaryNode };
		for (const prop in this.node_data.c) {
			const propkey = parseInt(prop);
			const childnode: BinaryNode = new BinaryNode(
				this.table_meta,
				this.column_meta,
				this.node_data.c[propkey],
				propkey,
				this
			);
			new_children[propkey] = childnode;
		}
		this.children = new_children;

		//console.debug("Build Binary Node " + this.fullkey);
	}

	isHidden() {
		return this.node_data?.h;
	}

	getColumnMeta() {
		return this.column_meta;
	}

	getTopNode() {
		if (!this.parent_node) {
			return this;
		}
		let current_node = this.parent_node;
		while (current_node.parent_node) {
			current_node = current_node.parent_node;
		}
		return current_node;
	}

	getLabel() {
		let label = '';
		if (this.node_data && this.node_data.l !== undefined) {
			const meta = this.getColumnMeta();
			if (meta) {
				label = meta.labels.map[this.node_data.l];
			}
		}
		return label;
	}

	clone() {
		return new BinaryNode(
			this.table_meta,
			this.column_meta,
			this.node_data,
			this.local_key,
			this.parent_node
		);
	}

	createDisplayNode(syncer: DisplaySync): BinaryNodeDisplay | null {
		if (!this.node_data) {
			return null;
		}
		console.debug('Creating display node for Binary Node', this, this.node_data.d);

		const column_meta = this.getColumnMeta();
		if (!column_meta) {
			return null;
		}
		let innerhtml: string = '';

		const child_nodes = [] as any[];
		let custom_display = null;

		//let label = this.getLabel();

		if (!this.node_data.h) {
			//Only show if it isn't hidden
			console.debug('Processing Staging Data');
			innerhtml += this.getLabel() + ': ';
			if (this.node_data.v !== undefined) {
				//Basic Display
				switch (this.node_data.d) {
					case DataType.LocalDate:
						{
							innerhtml += shortDateString(epochDayToLocalDate(this.node_data.v as number));
						}
						break;
					default: {
						let stringval: string = '';
						if (this.node_data.t !== undefined) {
							if (column_meta.translators[this.node_data.t] !== undefined) {
								if (typeof this.node_data.v) {
								}
								stringval = column_meta.translators[this.node_data.t][this.node_data.v];
							} else {
								console.error('Unpopulated translator.', this.node_data);
							}
						} else {
							stringval = this.node_data.v.toString();
						}
						innerhtml += stringval;
					}
				}
			}

			//Segment Display
			let ckey: number = 0;
			if (this.node_data.c != undefined) {
				switch (
					this.node_data.l
					/*
                    case BF_Label.AssignmentSegments:
                        {
                            innerhtml="";
                            addSegmentsToSyncer(syncer,this.node_data.c);
                            const props:AssignmentSegmentDisplayProps = {
                                key:(ckey++).toString(),
                                members:this.node_data.c,
                                column_meta:column_meta,
                                syncer:syncer,
                                individual_rows:false,
                                segment_select_hook:undefined
                            };
                            //custom_display = React.createElement(AssignmentSegmentDisplay,props,null);
                            //custom_style={width:"100%"};
                            //TODO
                        }
                        break;
                    case BF_Label.ScheduleTemplateDetails:
                        {
                            innerhtml="";
                            const props:ScheduleTemplateDisplayProps = {
                                key:(ckey++).toString(),
                                data:new ScheduleTemplateDetails(this.node_data),
                                column_meta:column_meta,
                                table_meta:this.table_meta,
                                assignment_select_hook:undefined,
                                constraint_select_hook:undefined
                            };
                            //custom_display = React.createElement(ScheduleTemplateDisplay,props,null);
                            //custom_style={width:"100%"};
                            //TODO
                        }
                        break;
                    case BF_Label.SingularRequest:
                    case BF_Label.RecurringRequest:
                        {
                            innerhtml="";
                            //custom_display=createRequestDisplay(this);
                            //TODO
                        }
                        break;
                    
                    default:
                        {
                            for(const child_node_key in this.children)
                            {
                                const child_node = this.children[child_node_key];
                                const child_result:BinaryNodeDisplay|null = child_node.creat   return GenericInputNode(node);eDisplayNode(syncer);
                                if(child_result && child_result.custom_display)
                                {
                                    custom_display=child_result.custom_display;
                                }
                            }
                        }
                        break;
                    */
					// TODO
				) {
				}
			}
		}

		return {
			custom_display: custom_display,
			children: child_nodes
		};
	}

	createRootInputNode(): any {
		//console.debug("Creating root input node for Binary Node",this);
		//return React.createElement(createRootNode,{key:"in " + this.local_key, node:this},null);
		//TODO
	}
}

export const createChildInputNode = (node: BinaryNode) => {
	//console.debug("Rendering child " + props.node.fullkey);
	if (!node.isHidden() && node.node_data) {
		return pickNode(node);
	} else {
		return null;
	}
};

const pickNode = (node: BinaryNode) => {
	if (!node.node_data) {
		return null;
	}

	/*
    if(node.node_data.l==BF_Label.AssignmentSegments)
    {
        //retval = React.createElement(AssignmentSegmentDisplayInputNode,passedprops,null);
        return AssignmentSegmentDisplayInputNode(node);
    }
    else if(node.node_data.l==BF_Label.ScheduleTemplateDetails)
    {
        //retval = React.createElement(ScheduleTemplateDetailsInputNode,passedprops,null);
        return ScheduleTemplateDetailsInputNode(node);
    }
    else if(node.node_data.l==BF_Label.RecurringRequest 
        || node.node_data.l==BF_Label.SingularRequest 
        //|| node.node_data.l==BF_Label.ChildRequest //not handled on singular or recurring
        //|| node.node_data.l==BF_Label.RecurringRequestDetails //nope
        //|| node.node_data.l==BF_Label.RecurringRequestTimingDetails //unhandled case
        || node.node_data.l==BF_Label.RecurringRequestTiming //not handled
        || node.node_data.l==BF_Label.RecurringRequestChild //not handled
        
        )
    {
        return RequestInputNode(node);
    }
    else
    {
        //retval = React.createElement(GenericInputNode,passedprops,null);
        return GenericInputNode(node);
    }
    */
	// TODO
};
