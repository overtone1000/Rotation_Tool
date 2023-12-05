<script lang="ts">
	import { stagingProcessedMembers } from '../stores';

	const count = $stagingProcessedMembers!.response.update_data.commitable.length;
	const date = '(this should be populated from the server as min date)';
	console.error('To do');
</script>

<div>
	<p>
		Assigned, locked assignments prior to today's date that are not entailed with the schedule may
		be committed.
	</p>
	<p>
		There are {count} commitable items.
	</p>
	<p>
		After a commit, staging will not be able to be performed prior to {date}.
	</p>
</div>

<style>
	div {
		margin: 10px;
	}
</style>
