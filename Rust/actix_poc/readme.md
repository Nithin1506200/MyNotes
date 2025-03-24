# INTRO

docs :

- <https://docs.google.com/document/d/1ciqUKH8pQEFL-KD5MLGprEVI-fiTKwD4a4CYBrA27Ow>

  Create an payment mocking system with Postgresql + redis

- cargo install diesel_cli --no-default-features --features postgres # Change to mysql/sqlite if needed
- psql -h localhost -p 5432 -U postgres -c "CREATE DATABASE payments_poc;"
- Setup DATABASE_URL=postgres://postgres:postgres@localhost:5432/payments_poc
- psql -h localhost -p 5432 -U postgres -d payments_poc

How to generate a migration

- export DATABASE_URL=postgres://postgres:postgres@localhost:5432/payments_poc
- diesel migration run
- diesel migration generate merchant_account
- diesel print-schema > src/schema.rs to print schema
- diesel migration revert --all
