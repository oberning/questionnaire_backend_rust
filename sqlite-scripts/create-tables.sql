-- questionnaire definition

CREATE TABLE questionnaire (
	id integer primary key autoincrement,
	"name" varchar NOT NULL
);

-- question definition

CREATE TABLE question (
	id integer primary key autoincrement,
	"text" varchar NOT NULL,
	max_score int4 NOT NULL,
	questionnaire_id int4 NOT NULL,
	FOREIGN KEY (questionnaire_id) REFERENCES questionnaire(id)
);

-- answer definition

CREATE TABLE answer (
	id integer PRIMARY KEY AUTOINCREMENT,
	"text" varchar NOT NULL,
	is_correct bool NOT NULL,
	question_id int4 NOT NULL,
	FOREIGN KEY (question_id) REFERENCES question(id)
);