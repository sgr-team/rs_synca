# Tests environment
docker compose -f ./tests/docker-compose.yml up -d

# synca
cargo test -p synca

# pg_as_calc
cargo test -p synca_example_pg_as_calc --features=sync,tokio