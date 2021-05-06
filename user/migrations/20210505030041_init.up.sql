
-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS user_table(
	id uuid default uuid_generate_v4(),
	name varchar NOT NULL,
	email varchar NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
	PRIMARY KEY (id)
);

insert into user_table(id,name,email)values('bccdd9f9-c2e8-49dc-9ba1-5cc992d8ada2','user_1','user_1@tacogips_test.com');
insert into user_table(id,name,email)values('25f8ea00-52f4-4c58-b8db-8b7075982139','user_2','user_2@tacogips_test.com');

