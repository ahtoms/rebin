CREATE TABLE source(
	source_id INTEGER,
	username TEXT NOT NULL,
	text TEXT NOT NULL,
	lang TEXT NOT NULL,
	ts INTEGER NOT NULL
);


CREATE TABLE comments(
	comment_id INTEGER PRIMARY KEY AUTOINCREMENT,
	sid INTEGER NOT NULL,
	response TEXT NOT NULL,
	line_start INTEGER NOT NULL,
	line_end INTEGER NOT NULL,
	ts INTEGER NOT NULL,
	FOREIGN KEY(sid) REFERENCES source(source_id)
);

INSERT INTO source(source_id, username, text, lang, ts) VALUES (0, 'Anonymous',
	'print("hello world")',
	'\npython',
	1585852329);
