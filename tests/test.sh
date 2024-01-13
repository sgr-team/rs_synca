docker compose -f ./tests/docker-compose.yml up -d
cargo test -p synca
cargo test -p synca_example_pg_as_calc --no-default-features --features=sync
cargo test -p synca_example_pg_as_calc --no-default-features --features=tokio