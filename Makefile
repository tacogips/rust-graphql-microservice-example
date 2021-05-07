mig-db:
	cd article; make db-up-mig;
	cd user; make db-up-mig;
	cd comment; make db-up-mig;

mig-down-db:
	cd article; make db-down-mig;
	cd user; make db-down-mig;
	cd comment; make db-down-mig;

refresh-db: mig-down-db mig-db

prepare-db:
	cd article; make sqlx-prepare;
	cd user; make sqlx-prepare;
	cd comment; make sqlx-prepare;


