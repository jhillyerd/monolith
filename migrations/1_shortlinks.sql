create table shortlinks (
  link_id uuid primary key default gen_random_uuid(),
  name text unique not null,
  url text unique not null
);
