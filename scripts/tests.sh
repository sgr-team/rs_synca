# Tests environment
docker compose -f ./scripts/tests.yml up -d

# sleep
sleep 5

# synca
cargo test -p synca

# pg_as_calc
cargo test -p synca_example_pg_as_calc --features=sync,tokio