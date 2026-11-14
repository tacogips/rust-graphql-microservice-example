
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


insert into article(id,status,text,author_id)values('9859dfe9-6442-4929-baca-865e669741e5','published','this is a text docuemnt\n hello','bccdd9f9-c2e8-49dc-9ba1-5cc992d8ada2');
insert into article(id,status,text,author_id)values('aece92fe-3b06-4ec6-a84f-57704c3d52db','draft','this is a draft','25f8ea00-52f4-4c58-b8db-8b7075982139');


