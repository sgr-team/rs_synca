# Tests environment
docker compose -f ./tests/docker-compose.yml up -d

# synca
cargo test -p synca

# pg_as_calc
cargo test -p synca_example_pg_as_calc --no-default-features --features=sync
cargo test -p synca_example_pg_as_calc --no-default-features --features=tokio