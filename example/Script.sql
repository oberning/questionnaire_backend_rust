-- TODO: use select last_value from question_id_seq;
-- or try
-- /* select nextval('question_id_seq'); this happens in the session automatically */ 
-- select currval('question_id_seq');

INSERT INTO questionnaire
("name")
VALUES('Heise Quiz');

INSERT INTO public.question
("text", max_score, questionnaire_id)
VALUES('Meta and Google work on language models. What will they be able?', 2, 1);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Answer messages automatically', false, 4);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Laughing', false, 4);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Learn how to use APIs', true, 4);

INSERT INTO public.question
("text", max_score, questionnaire_id)
VALUES('What is the situation about copyright for ChatGPT and others?', 2, 1);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Copyrights do not apply to them', false, 5);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('The AI copyright law regulates that', false, 5);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('It is difficult and unclear', true, 5);

INSERT INTO public.question
("text", max_score, questionnaire_id)
VALUES('Why is the Internet Explorer dead?', 2, 1);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Because there is no AI in it', false, 6);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('It was replaced by Edge', true, 6);

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Language models cannot capture data', true, 6);



INSERT INTO public.question
("text", max_score, questionnaire_id)
VALUES('What is Copilot able to do?', 2, (select last_value from questionnaire_id_seq));

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Pretend conversation', false, (select last_value from question_id_seq));

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Colorising', true, (select last_value from question_id_seq));

INSERT INTO public.answer
("text", is_correct, question_id)
VALUES('Coding', true, (select last_value from question_id_seq));