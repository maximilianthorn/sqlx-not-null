# Steps to reproduce
- add `.env` file containing `DATABASE_URL="postgres://<USER>:<PASSWORD>@<HOST>:<PORT>/<DB>"`
- run:
```sh
sqlx database create
sqlx migrate run
```

Now, `cargo build` should run successfully, `cargo run` should crash.
