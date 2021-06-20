export interface Resource {
	name: string;
	description: string;
	status: string;
	other_fields: {
		[index: string]: string;
	};
}
