ALTER TABLE books
	ADD CONSTRAINT unique_path UNIQUE (path),
	DROP CONSTRAINT books_name_check,
	DROP CONSTRAINT books_name_key;

ALTER TABLE books
	ADD CONSTRAINT books_name_check CHECK (length(title) >= 1);
