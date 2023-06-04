-- TODO: use select last_value from question_id_seq;
-- or try
-- /* select nextval('question_id_seq'); this happens in the session automatically */ 
-- select currval('question_id_seq');

INSERT INTO questionnaire
("name")
VALUES('Heise Quiz');



INSERT INTO question
("text", max_score, questionnaire_id)
VALUES('Meta and Google work on language models. What will they be able?', 2, (select max(id) from questionnaire));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Answer messages automatically', false, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Laughing', false, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Learn how to use APIs', true, (select max(id) from question));



INSERT INTO question
("text", max_score, questionnaire_id)
VALUES('What is the situation about copyright for ChatGPT and others?', 2, (select max(id) from questionnaire));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Copyrights do not apply to them', false, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('The AI copyright law regulates that', false, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('It is difficult and unclear', true, (select max(id) from question));



INSERT INTO question
("text", max_score, questionnaire_id)
VALUES('Why is the Internet Explorer dead?', 2, (select max(id) from questionnaire));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Because there is no AI in it', false, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('It was replaced by Edge', true, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Language models cannot capture data', true, (select max(id) from question));



INSERT INTO question
("text", max_score, questionnaire_id)
VALUES('What is Copilot able to do?', 2, (select max(id) from questionnaire));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Pretend conversation', false, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Colorising', true, (select max(id) from question));

INSERT INTO answer
("text", is_correct, question_id)
VALUES('Coding', true, (select max(id) from question));