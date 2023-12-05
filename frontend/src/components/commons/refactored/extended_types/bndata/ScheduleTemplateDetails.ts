import { AssignmentMember } from './AssignmentMember';
import { Constraint } from './Constraint';
import { ExtendedBNData } from './ExtendedBNData';

export class ScheduleTemplateDetails extends ExtendedBNData {
	getAssignmentMembersData = () => {
		return this.data.c[0].c;
	};
	getConstraintMembersData = () => {
		return this.data.c[1].c;
	};
	getAssignmentMembers = () => {
		const retval: AssignmentMember[] = [];
		for (const smd_i in this.getAssignmentMembersData()) {
			const smd = this.getAssignmentMembersData()[smd_i];
			retval.push(new AssignmentMember(smd));
		}
		return retval;
	};
	getConstraintMembers = () => {
		const retval: Constraint[] = [];
		for (const cmd_i in this.getConstraintMembersData()) {
			const cmd = this.getConstraintMembersData()[cmd_i];
			retval.push(new Constraint(cmd));
		}
		return retval;
	};
}
