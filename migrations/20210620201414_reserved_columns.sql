BEGIN;
ALTER TABLE resources ADD COLUMN reserved_until BIGINT default 0 NOT NULL;
ALTER TABLE resources ADD COLUMN reserved_by TEXT default '' NOT NULL;
COMMIT;
