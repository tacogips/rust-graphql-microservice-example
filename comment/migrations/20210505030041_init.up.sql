
-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS comment_table(
	id uuid default uuid_generate_v4(),
	text text NOT NULL,
	article_id uuid not null,
	author_id uuid not null,
  created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
	PRIMARY KEY (id)
);

insert into comment_table(text,author_id,article_id)values('comment_1','9859dfe9-6442-4929-baca-865e669741e5','bccdd9f9-c2e8-49dc-9ba1-5cc992d8ada2');
insert into comment_table(text,author_id,article_id)values('comment_2','9859dfe9-6442-4929-baca-865e669741e5','25f8ea00-52f4-4c58-b8db-8b7075982139');
insert into comment_table(text,author_id,article_id)values('comment_3','9859dfe9-6442-4929-baca-865e669741e5','bccdd9f9-c2e8-49dc-9ba1-5cc992d8ada2');
insert into comment_table(text,author_id,article_id)values('comment_4','aece92fe-3b06-4ec6-a84f-57704c3d52db','25f8ea00-52f4-4c58-b8db-8b7075982139');



