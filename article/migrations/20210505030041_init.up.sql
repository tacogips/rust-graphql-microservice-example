
-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE article_status AS ENUM ('published','draft');

CREATE TABLE IF NOT EXISTS article(
	id uuid default uuid_generate_v4(),
	status article_status NOT NULL,
	text varchar,
	author_id uuid NOT NULL,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
	PRIMARY KEY (id)
);

insert into article(status,text,author_id)values('published','this is a text docuemnt\n hello','bccdd9f9-c2e8-49dc-9ba1-5cc992d8ada2');
insert into article(status,text,author_id)values('draft','this is a draft','25f8ea00-52f4-4c58-b8db-8b7075982139');

