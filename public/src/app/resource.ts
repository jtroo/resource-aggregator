export interface Resource {
	name: string;
	description: string;
	reserved_until: number;
	reserved_by: string;
	other_fields: {
		[index: string]: string;
	};
}
