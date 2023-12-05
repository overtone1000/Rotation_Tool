export const getValues = (values: number[]) => {
	const retval = [] as string[];
	for (const num of values) {
		retval.push(num.toString());
	}
	return retval;
};

export const getTargetValues = (targetvalue: string[]) => {
	const retval = [] as number[];
	for (const val of targetvalue) {
		retval.push(parseInt(val));
	}
	return retval;
};
