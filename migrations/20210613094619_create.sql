BEGIN;
CREATE TABLE resources (
	name TEXT PRIMARY KEY,
	status TEXT NOT NULL,
	description TEXT NOT NULL,
	other_fields JSONB NOT NULL
);
END;
