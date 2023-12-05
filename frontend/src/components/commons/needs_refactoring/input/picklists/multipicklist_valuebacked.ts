'use strict';

import { ObjectKeyPair } from '../form_elements';

export const createValueBackedMultiplePickList = (
	label: string,
	disabled: boolean,
	value_object: {},
	value_key: string,
	options: { [key: number]: string },
	changeHandler: undefined | (() => void),
	order: (number | string)[]
): React.ReactNode => {
	const vop = new ObjectKeyPair<number[]>(value_object, value_key);
	/* return (
        <ValueBackedMultiplePickList
            key={value_key}
            label={label}
            disabled={disabled}
            objectkeypair={vop}
            options={options}
            order={order}
            changeHandler={changeHandler}
        />
    )
    */
	return null;
	// TODO
};

/*
export interface ValueBackedMultiplePickList_Props extends FilterSelectPickList_Props
{
  objectkeypair:ObjectKeyPair<number[]>,
  options:{[key:number]:string},
  order:(number|string)[]
}

export const ValueBackedMultiplePickList:FC<ValueBackedMultiplePickList_Props> = (props:ValueBackedMultiplePickList_Props) =>
{
  console.debug("Creating vbmpl, props",props);
  const state:MultiPickListState = constructMultiPickListState(props.options, props.order, props.objectkeypair.get());

  const updatedChangeHandler = (newvals:number[])=>{
    if(props.changeHandler!==null && props.changeHandler!==undefined)
    {
      props.changeHandler(newvals);
    }
    props.objectkeypair.set(newvals);
    state.selected_values.set(newvals);
  }


  const renderValue = (value:string[]) =>
  {
    console.debug("renderValue",value);
    let retval:string = null;
    for(const s of value)
    {
      if(retval===null)
      {
        retval = props.options[s];
      }
      else
      {
        retval = retval + ", " + props.options[s];
      }
    }
    return retval;
  }


  //<FilterSelectPickList
  //  label={props.label}
  //  options={props.options}
  //  disabled={props.disabled}
  //  changeHandler={updatedChangeHandler}/>

    //

    //style = {{height:"100%"}}

  let label_id="label";
  if(props.label){label_id=props.label+label_id;}
  const value = getValues(props.objectkeypair.get());
  console.debug("value",value);
  //
  //
  //label={props.label}

  return (
    <FormControl sx={formStyle_generic}>
      <InputLabel id={label_id}>{props.label}</InputLabel>
      <Select 
          labelId={label_id}
          label={props.label}
          id="vb_select"
          multiple
          disabled={props.disabled} 
          value={value} 
          MenuProps={MenuProps}
          renderValue={renderValue}
          onChange = {(evt)=>{console.debug("Evt is",evt.target.value);updatedChangeHandler(getTargetValues(evt.target.value as string[]));}}
          input={<OutlinedInput label="Tag" />}
          >
          {state.keyvalobjects}
      </Select>
    </FormControl>
  );
}
*/
