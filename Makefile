mig-db:
	cd article; make db-up-mig;
	cd user; make db-up-mig;
	cd comment; make db-up-mig;

prepare-db:
	cd article; make sqlx-prepare;
	cd user; make sqlx-prepare;
	cd comment; make sqlx-prepare;

