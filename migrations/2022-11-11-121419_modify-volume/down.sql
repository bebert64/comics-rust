ALTER TABLE volumes
ADD number INTEGER NOT NULL;

ALTER TABLE volumes
DROP title;

ALTER TABLE volumes
DROP publisher;

ALTER TABLE volumes
DROP year_start
