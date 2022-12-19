-- Add down migration script here
alter table requests drop column if exists status;
