DATABASE_URL=postgres://db_user:pass@localhost:5432/user_db
db-create:
	DATABASE_URL=${DATABASE_URL} sqlx database create

db-info-mig:
	DATABASE_URL=${DATABASE_URL} sqlx mig info

db-up-mig:
	DATABASE_URL=${DATABASE_URL} sqlx mig run

db-down-mig:
	DATABASE_URL=${DATABASE_URL} sqlx mig revert


change-db-env:
	export DATABASE_URL=${DATABASE_URL}

sqlx-prepare:
	cargo clean; DATABASE_URL=${DATABASE_URL} cargo sqlx prepare -- --bin user-server


