# entgen

entgen is cli tool to generate entity and repository for [sqlx](https://github.com/launchbadge/sqlx). This project is work-in-progress. Stay tuned.

## Usage

```:bash
$ target/debug/entgen --help
entgen 0.0.1
Hikaru Miyahara
Entity generator for sqlx

USAGE:
    entgen [OPTIONS]

OPTIONS:
    -f, --file <FILE>    Set entgen config file [default: entgen.toml]
    -h, --help           Print help information
    -V, --version        Print version information
```

## Config

You must define entgen.toml file if you use entgen cli.
Config file format is below.

```:toml
output_dir = "src/entity"

[postgres]
user = "testuser"
password = "testpassword"
host = "localhost"
port = 5432
db = "testdb"
```

## DB model and Rust implementation

At first, you must define db model.

```:sql
CREATE TABLE users (
    id uuid PRIMARY KEY,
    name varchar(255) NOT NULL,
    nickname varchar(255),
    created_at timestamp NOT NULL
);
```

If you ware defined db model like above, you can get Rust model to use cli.

```:rs
#[derive(sqlx::FromRow, Debug)]
pub struct Users {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub nickname: Option<String>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

use std::ops::Deref;
use std::sync::Arc;

pub struct UsersRepository {
    pool: Arc<sqlx::PgPool>,
}

impl UsersRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        UsersRepository { pool: pool }
    }

    pub async fn insert(&self, users: &Users) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (id, name, nickname, created_at) VALUES ($1, $2, $3, $4)")
            .bind(users.id)
            .bind(users.name.clone())
            .bind(users.nickname.as_ref())
            .bind(users.created_at)
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Users>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Users>("SELECT * FROM users")
            .fetch_all(self.pool.deref())
            .await?)
    }
}
```

## Example

Example is [here](https://github.com/hikaru7719/entgen/tree/main/examples/sqlx-postgres).

## DB

This cli only supports PostgreSQL now.
