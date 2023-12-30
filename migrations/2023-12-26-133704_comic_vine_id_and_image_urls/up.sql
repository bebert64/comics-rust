ALTER TABLE books
    ADD COLUMN comic_vine_id integer,
    ADD COLUMN url_thumbnail VARCHAR,
    ADD COLUMN url_cover VARCHAR;

ALTER TABLE volumes
    ADD COLUMN comic_vine_id integer,
    ADD COLUMN url_thumbnail VARCHAR,
    ADD COLUMN url_cover VARCHAR;

ALTER TABLE issues
    ADD COLUMN comic_vine_id integer,
    ADD COLUMN url_thumbnail VARCHAR,
    ADD COLUMN url_cover VARCHAR;

ALTER TABLE reading_orders
    ADD COLUMN comic_vine_id integer,
    ADD COLUMN url_thumbnail VARCHAR,
    ADD COLUMN url_cover VARCHAR;
