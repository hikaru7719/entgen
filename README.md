# entgen

entgen is cli tool to generate entity for [sqlx](https://github.com/launchbadge/sqlx). This repository is work-in-progress. Stay tuned.

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

## Example

Example is [here](https://github.com/hikaru7719/entgen/tree/main/examples/sqlx-postgres).