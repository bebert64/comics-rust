ALTER TYPE archive_status RENAME VALUE 'found' TO 'to_unzip';

ALTER TYPE archive_status RENAME VALUE 'unzipped' TO 'to_parse';

ALTER TYPE archive_status RENAME VALUE 'parsed_type' TO 'to_parse_issues';

ALTER TYPE archive_status RENAME VALUE 'parsed_info' TO 'to_complete_issues';

ALTER TYPE archive_status RENAME VALUE 'has_comic_vine_id' TO 'to_search_comic_vine_id';

ALTER TYPE archive_status
	ADD VALUE 'ok';
