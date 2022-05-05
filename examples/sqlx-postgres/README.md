# usage

- build entgen

```
cd ../../
cargo build
```

- docker compose up

```
cd examples/sqlx-postgres
docker compose up -d --build
```

- generate entity

```
rm -r src/entity
../../target/debug/entgen
```

- execute query

```
cargo run
```