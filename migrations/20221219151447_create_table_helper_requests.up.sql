-- Add up migration script here
create table if not exists helper_requests (
    id serial primary key,
    helper_id int,
    request_id int,

    constraint fk_helper foreign key(helper_id) references helpers(id),
    constraint fk_request foreign key(request_id) references requests(id),

    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    deleted_at timestamp
);
