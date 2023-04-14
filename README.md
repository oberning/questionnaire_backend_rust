# questionnaire_backend_rust
A Questionnaire e.g. for the first job interview

## Database Setup
* Install and configure PostgreSQL
* Create the database `questionnaire` so that it fits to the connection string `postgres://postgres:postgres@localhost:5432/questionnaire`

## Build and Run
* Build the API with `cargo build`
* Run via `cargo run`

## Example
* Execute the SQL Script `Script.sql` that is in the example folder

## Invoking this API
Follow the instructions assuming that you have [httpie](https://httpie.io) installed.

Get all available questionnaires with
```shell
http GET http://127.0.0.1:8080/questionnaires
```

Choose a questionnaire and get all questions:
```shell
http GET http://127.0.0.1:8080/questionnaire/1
```

Choose a question and get all answers:
```shell
http GET http://127.0.0.1:8080/question/2
```


## TO-DO
The `questionnaire` endpoint is too verbose and contains properties that already exists in the response of the `question` endpoint.

Idea: the `questions` node of the `questionnaire` endpoint only contains a list of question IDs. Additionally, it might contain the `score` field with the sum of all `max_score`.