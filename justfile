export DATABASE_URL := "postgres://user:pass@localhost/mydb"

# list recipes
default:
    @just --list

# build all workspace targets
build-all:
    cargo build --all

# run migration script
migrate *ARGS: build-all
    ./target/debug/migration {{ ARGS }}

# update entities sources files from database
update_entities:
    sea-orm-cli generate entity -u $DATABASE_URL -o ./entity/src/

# run server debug build
run-server:
    cargo run

# run server release build
run-server-release:
    cargo run --release --bin server
