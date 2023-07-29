--
-- create schema
create schema if not exists jen;

--
-- create uuid extension
create extension if not exists "uuid-ossp" schema jen;

--
-- search path
set search_path to jen;

--
-- update timestamp function
create or replace function update_timestamp()
  returns trigger
  as $$
begin
  new.updated_at = current_timestamp;
  return new;
end;
$$
language plpgsql;

--
-- hash algorithms type
create type hash_algorithm as enum(
  'argon2',
  'bcrypt',
  'scrypt',
);

--
-- users table
create table if not exists users(
  id uuid not null default uuid_generate_v4() primary key,
  first_name text not null,
  last_name text not null,
  email text not null,
  username text not null,
  image_uri text not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (email)
);

create or replace trigger update_users_timestamp
  before update on users for each row
  execute function update_timestamp();

--
-- user_credentials table
create table if not exists user_credentials(
  id uuid not null default uuid_generate_v4() primary key,
  user_id uuid not null references users(id) on delete cascade,
  credential_hash text not null,
  alg hash_algorithm not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);

create or replace trigger update_user_credentials_timestamp
  before update on user_credentials for each row
  execute function update_timestamp();

create table if not exists spaces(
  id uuid not null default uuid_generate_v4() primary key,
  space_name text not null,
  bio text not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (space_name)
);

create or replace trigger update_spaces_timestamp
  before update on spaces for each row
  execute function update_timestamp();

--
-- posts table
create table if not exists posts(
  id uuid not null default uuid_generate_v4() primary key,
  user_id uuid not null references users(id) on delete cascade,
  space_id uuid not null references spaces(id) on delete cascade,
  title text not null,
  content text not null,
  read_time int not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);

create or replace trigger update_posts_timestamp
  before update on posts for each row
  execute function update_timestamp();

--
-- tags table
create table if not exists tags(
  id uuid not null default uuid_generate_v4() primary key,
  space_id uuid not null references spaces(id) on delete cascade,
  tag_name text not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (tag_name)
);

create or replace trigger update_tags_timestamp
  before update on tags for each row
  execute function update_timestamp();

--
-- post_tags table
create table if not exists post_tags(
  id uuid not null default uuid_generate_v4() primary key,
  post_id uuid not null references posts(id) on delete cascade,
  tag_id uuid not null references tags(id) on delete cascade,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);

create or replace trigger update_post_tags_timestamp
  before update on post_tags for each row
  execute function update_timestamp();

