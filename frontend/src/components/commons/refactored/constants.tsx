//correlates with BinayMetaProvider.java
export enum BF_Label {
	AssignmentSegments = 7,
	AssignmentDetails = 8,
	AssignmentMember = 14,
	AssignmentMembers = 15,
	ConstraintMember = 17,
	ConstraintMembers = 18,
	ScheduleTemplateDetails = 21,
	ConstraintAssignmentMember = 22, //22 //Needs to be different for the Javascript display stuff
	ConstraintMultiassignmentMember = 23, //23 //Needs to be different for the Javascript display stuff
	Workers = 24, //24 //Needs to be different for the Javascript display stuff

	Date = 27, //27

	DetailsType = 31,

	//Requests
	RequestType = 33,
	SingularRequest = 34,
	SingularRequestDetails = 35,
	RecurringRequest = 36,
	RecurringRequestTiming = 37,
	RecurringRequestTimingType = 38,
	RecurringRequestTimingDetails = 39,
	RecurringRequestType = 40,
	RecurringRequestDetails = 41,
	RecurringRequestChildren = 42,
	RecurringRequestChild = 46
}
