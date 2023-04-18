ALTER TABLE issues RENAME TO old_issues;

CREATE TABLE issues
(
    id INTEGER NOT NULL PRIMARY KEY,
    is_read BOOLEAN NOT NULL,
    number INTEGER NOT NULL,
    cover_date DATE,
    thumbnail BLOB,
    volume_id INTEGER,
    comic_vine_id INTEGER,
    book_id INTEGER,
    author_id INTEGER,
    artist_id INTEGER,
    path VARCHAR (255),
    UNIQUE (path)
);

INSERT INTO issues SELECT * FROM old_issues;

DROP TABLE old_issues
