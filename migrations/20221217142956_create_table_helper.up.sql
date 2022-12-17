-- Add up migration script here
create table if not exists helpers (
    id serial primary key,
    name varchar(255),
    phone_number varchar(25),
    email varchar(100),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    deleted_at timestamp
);