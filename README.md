# RUST Actix + Diesel + Postgresql + Cronjob

## Description
This is a Rust actix diesel# RUST Actix + Diesel + Postgresql + Cron

to run the project you need to have the diesel cli installed on the system.

```bash
cd simple-cron-rust
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel migration run
cargo run
```


