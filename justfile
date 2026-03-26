export DATABASE_URL := "postgres://user:pass@localhost/mydb"

default:
    @just --list

migrate *ARGS:
    cargo run --bin migrate -- {{ ARGS }}

update_entities:
    sea-orm-cli generate entity -u $DATABASE_URL -o ./src/entities/

run-server:
    cargo run --bin server

run-server-release:
    cargo run --release --bin server

start-postgres:
    docker run --rm -d \
      --name my-postgres \
      -e POSTGRES_USER=user \
      -e POSTGRES_PASSWORD=pass \
      -e POSTGRES_DB=mydb \
      -p 5432:5432 \
      postgres:latest
