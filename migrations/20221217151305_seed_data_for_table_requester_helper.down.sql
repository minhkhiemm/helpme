-- Add down migration script here
update requesters set deleted_at = now() where name = 'Alice';
update helpers set deleted_at = now() where name = 'Bob';