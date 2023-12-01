psql:
	docker exec -it pocket-change-problem-db-1 psql -U postgres -h localhost -p 5432
test:
# Errors occur when it executes the tests in parallel.
# https://github.com/launchbadge/sqlx/issues/2631
	docker compose exec backend cargo test -- --test-threads=1