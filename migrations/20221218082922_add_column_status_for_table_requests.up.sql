-- Add up migration script here
alter table requests add column if not exists status varchar(50) default 'new' not null;
