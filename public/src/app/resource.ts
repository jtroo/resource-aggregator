export interface Resource {
	name: string;
	description: string;
	status: string;
	reserved_until: number;
	reserved_by: string;
	other_fields: {
		[index: string]: string;
	};
}
