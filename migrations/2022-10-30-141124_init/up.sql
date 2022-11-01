CREATE TABLE comics (
    id INTEGER NOT NULL PRIMARY KEY,
	title VARCHAR (255) NOT NULL,
	thumbnail BLOB,
	publisher_id INTEGER,
    comic_vine_id INTEGER
);

CREATE TABLE volumes (
    id INTEGER NOT NULL PRIMARY KEY,
	number INTEGER NOT NULL,
	thumbnail BLOB,
    comic_id INTEGER NOT NULL,
    comic_vine_id INTEGER
);

CREATE TABLE story_arcs (
    id INTEGER NOT NULL PRIMARY KEY,
	title VARCHAR (255) NOT NULL,
	thumbnail BLOB,
    comic_vine_id INTEGER
);

CREATE TABLE books (
    id INTEGER NOT NULL PRIMARY KEY,
	title VARCHAR (255) NOT NULL,
	thumbnail BLOB,
    is_tpb BOOLEAN NOT NULL
);

CREATE TABLE issues (
    id INTEGER NOT NULL PRIMARY KEY,
    is_read BOOLEAN NOT NULL,
    number INTEGER NOT NULL,
    cover_date DATE,
    thumbnail BLOB,
    volume_id INTEGER,
    comic_vine_id INTEGER,
    book_id INTEGER,
    author_id INTEGER,
    artist_id INTEGER
);

CREATE TABLE publishers (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    thumbnail BLOB,
    comic_vine_id INTEGER,
    UNIQUE (name)
);

CREATE TABLE creators (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    thumbnail BLOB
);
