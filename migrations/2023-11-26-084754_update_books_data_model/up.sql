ALTER TABLE books RENAME COLUMN name TO title;

ALTER TABLE books
	ADD COLUMN volume_id integer REFERENCES volumes(id),
	ADD COLUMN volume_number integer,
	DROP COLUMN book_type,
	DROP COLUMN path;

ALTER TABLE books
	ADD COLUMN path text NOT NULL;

COMMENT ON COLUMN books.volume_number IS 'Number of the TPB inside the volume. Ex: Batman v2 v03 => 3';
