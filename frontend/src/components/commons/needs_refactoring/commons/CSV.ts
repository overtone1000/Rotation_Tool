const toCSVSafeString = (s: string) => {
	return '"' + s + '"';
};
const comma = ',';
const return_carriage = '\r';
const new_line = '\n';
const doublequote = '"';

export class CSV_Builder {
	private output: string;

	constructor() {
		this.output = '';
	}

	public addStringField(field: string) {
		this.output += toCSVSafeString(field) + comma;
	}

	public addNumericField(field: number) {
		this.output += field + comma;
	}

	public addEmptyField() {
		this.output += comma;
	}

	private trim_trailing() {
		while (
			this.output[this.output.length - 1] === comma ||
			this.output[this.output.length - 1] === new_line
		) {
			this.output = this.output.substring(0, this.output.length - 1);
		}
	}

	public newRow() {
		this.trim_trailing();
		this.output += new_line;
	}

	public getResult() {
		this.trim_trailing();
		return this.output;
	}
}

type ParsedCSV = string[][];
export const parseCSV: (csv: string) => ParsedCSV = (csv: string) => {
	const retval: ParsedCSV = [];
	const raw_rows = csv.replaceAll(return_carriage, '').split(new_line);

	console.debug(csv);
	console.debug(csv.replaceAll(return_carriage, ''));
	console.debug(raw_rows);

	for (const row_index in raw_rows) {
		const parsed_row = [] as string[];
		retval.push(parsed_row);

		let escaped: boolean = false;
		let row = raw_rows[row_index];

		let start_index = 0;

		for (let n = 0; n <= row.length; n++) {
			if (n === row.length) {
				console.debug('End of row!');
			}
			const char = row[n];
			if (char === doublequote) {
				escaped = !escaped;
			} else if ((char === comma && !escaped) || n === row.length) {
				let val = '';

				if (n - start_index > 0) {
					val = row.substring(start_index, n);
					//Remove escaped quotes
					if (
						val.substring(0, doublequote.length) === doublequote &&
						val.substring(val.length - doublequote.length, val.length) === doublequote
					) {
						val = val.substring(doublequote.length, val.length - doublequote.length);
					}
				}

				parsed_row.push(val);

				start_index = n + 1;
			}
		}
		console.debug(parsed_row);
	}

	console.debug(retval);
	return retval;
};
