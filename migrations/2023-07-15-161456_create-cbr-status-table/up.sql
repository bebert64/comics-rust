CREATE TYPE zip_status AS enum(
	'Found',
    'Unzipped',
	'ParsedType',
	'ParsedInfo',
	'HasComicsVineId'
);

CREATE TABLE "zips"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	"path" text NOT NULL,
    "status" zip_status NOT NULL
);

CREATE TABLE "volumes"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "name" text NOT NULL UNIQUE CHECK(length("name") >= 3)
);

CREATE TABLE "issues"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "volume_id" integer REFERENCES volumes(id) NOT NULL,
    "number" integer NOT NULL,
	"dir" text,
    "status" zip_status NOT NULL
);

CREATE TABLE "books"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "name" text UNIQUE CHECK(length("name") >= 3)
);

CREATE TABLE "books_issues"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "bookd_id" integer NOT NULL REFERENCES books(id),
    "issue_id" integer NOT NULL REFERENCES issues(id)
);

CREATE TABLE "books_additional_files"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "bookd_id" integer NOT NULL REFERENCES books(id),
    "file_path" text NOT NULL UNIQUE CHECK(length("file_path") > 0)
);

CREATE TABLE "reading_orders"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "name" text UNIQUE CHECK(length("name") >= 3)
);

CREATE TABLE "reading_order_elements"(
	"id" integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "issue_id" integer REFERENCES issues(id),
    "book_id" integer REFERENCES books(id),
    "reading_order_id" integer REFERENCES reading_orders(id) CHECK(
        num_nonnulls(issue_id, book_id, reading_order_id) = 1)
);
