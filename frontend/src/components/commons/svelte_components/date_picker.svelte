<script lang="ts">
	import Textfield from '@smui/textfield';
	export let label: string;
	export let date: Date;
	export let max: Date = new Date(2999, 0);
	export let min: Date = new Date(1, 0);

	function dateToInputString(date: Date) {
		if (isNaN(date.getTime())) {
			date = new Date();
		}
		console.debug(date);
		const retval = date.toISOString().substring(0, 10);
		return retval;
	}
	function inputStringToDate(datestr: string) {
		const split = datestr.split('-');
		const retval = new Date(parseInt(split[0]), parseInt(split[1]) - 1, parseInt(split[2]));
		return retval;
	}

	let id = Math.random().toString();

	let datestr: string;
	$: {
		datestr = dateToInputString(date);
	}

	const dateChange = (e: any) => {
		date = inputStringToDate(datestr);
	};
</script>

<div>
	<Textfield
		{id}
		{label}
		type="date"
		required
		on:change={dateChange}
		bind:value={datestr}
		min={dateToInputString(min)}
		max={dateToInputString(max)}
	/>
</div>
