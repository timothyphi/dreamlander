# dreamlander

Living the dream

## Runs on file change

```shell
cargo watch -x check -x clippy -x test -x run
```

## Runs on push

```shell
cargo build
cargo test
cargo clippy
cargo fmt
```

## Runs everyday at midnight

```shell
cargo audit
```
