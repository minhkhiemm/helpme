-- Add up migration script here
create table if not exists requests (
    id serial primary key,
    title varchar(255),
    description text,
    price bigint,
    helper_id int,
    requester_id int,

    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    deleted_at timestamp,

    constraint fk_helper foreign key(helper_id) references helpers(id),
   constraint fk_requester foreign key(requester_id) references requesters(id) 
);