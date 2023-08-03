CREATE TYPE book_type AS enum(
	'graphic_novel',
	'single_volume',
	'multi-volume'
);

ALTER TABLE "archives"
	ADD CONSTRAINT unique_archives_path UNIQUE ("path");

ALTER TABLE "issues" RENAME COLUMN "dir" TO "path";

ALTER TABLE "issues"
	ADD CONSTRAINT "unique_issue_number_in_volume" UNIQUE ("volume_id", "number"),
	ADD CONSTRAINT "unique_issue_path" UNIQUE ("path");

ALTER TABLE "books"
	ADD COLUMN "path" text UNIQUE,
	ADD COLUMN "type" book_type NOT NULL,
	ADD CONSTRAINT "multi_volume_books_and_graphic_novels_must_have_a_name" CHECK ("type" = single_volume OR "name" IS NOT NULL);

ALTER TABLE "books_issues" RENAME TO "books__issues";

ALTER TABLE "books__issues" RENAME COLUMN "bookd_id" TO "book_id";

ALTER TABLE "books__issues"
	ADD CONSTRAINT "unique_books_issues" UNIQUE ("book_id", "issue_id"),
	ADD COLUMN "position" integer NOT NULL,
	ADD CONSTRAINT "unique_position_per_book" UNIQUE ("book_id", "position");

ALTER TABLE "books_additional_files" RENAME TO "books__additional_files";

ALTER TABLE "books__additional_files" RENAME COLUMN "bookd_id" TO "book_id";

ALTER TABLE "reading_order_elements"
	DROP CONSTRAINT "reading_order_elements_check",
	DROP COLUMN "issue_id",
	ALTER COLUMN "reading_order_id" SET NOT NULL,
	ALTER COLUMN "book_id" SET NOT NULL,
	ADD COLUMN "position" integer NOT NULL,
	ADD CONSTRAINT "unique_position_per_reading_order" UNIQUE ("reading_order_id", "position");

ALTER TABLE "reading_order_elements" RENAME TO "reading_orders__books";
