# Evaluate Simple Stacks

I have a on site coming up, and I need to evaluate stacks for use in the interview. The frontend[^1] and database[^2] have been selected. What's being evaluated are the backend api tools.

## Evaluation Criteria

I'll be building a To Do list app with each backend. The frontend and database will be shared between all apps. For the backends I'll be evaluating:

- Endpoint Creation Ergonomics
- Using SQLite
  - Relational Database CRUD
  - Document Database CRUD
  - Key Value Database CRUD

## Language + API Library + SQL Library

- Rust + Actix Web + SQLx | SeaORM
  - port 8001
- Python(Mypy) + FastAPI + SQLAlchemy
  - port 8002
- Typescript + Express + Knex
  - port 8003

[^1]: React written in Typescript built with Vite.
[^2]: SQLite.
