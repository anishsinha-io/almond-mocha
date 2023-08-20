begin;
--
-- create schema
create schema if not exists jen;
--
-- search path
set search_path to jen;
--
-- create uuid extension
create extension if not exists "uuid-ossp";
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
-- hash_algorithm type
create type hash_algorithm as enum(
  'argon2',
  'bcrypt'
);
--
-- asset_backend type
create type asset_backend as enum(
  'fs',
  'aws',
  'gcp',
  'azure'
);
--
-- asset_visibility type
create type asset_visibility as enum(
  'public',
  'private'
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
--
-- spaces table
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
-- assets table
create table if not exists assets(
  id uuid not null default uuid_generate_v4() primary key,
  backend asset_backend not null default 'fs' ::asset_backend,
  file_path text not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (file_path)
);
create or replace trigger update_assets_timestamp
  before update on assets for each row
  execute function update_timestamp();
--
-- posts table
create table if not exists posts(
  id uuid not null default uuid_generate_v4() primary key,
  user_id uuid not null references users(id) on delete cascade,
  space_id uuid not null references spaces(id) on delete cascade,
  image_uri text not null,
  title text not null,
  content text not null,
  read_time int not null,
  visibility asset_visibility not null default 'public' ::asset_visibility,
  published boolean not null default false,
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
  tag_description text not null,
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
--
-- sessions table
create table if not exists sessions(
  id uuid not null default uuid_generate_v4() primary key,
  user_id uuid not null references users(id) on delete cascade,
  data jsonb not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);
create or replace trigger update_sessions_timestamp
  before update on sessions for each row
  execute function update_timestamp();
--
-- permissions table
create table if not exists permissions(
  id uuid not null default uuid_generate_v4() primary key,
  permission_name text not null,
  permission_description text not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (permission_name)
);
create or replace trigger update_permissions_timestamp
  before update on permissions for each row
  execute function update_timestamp();
--
-- roles table
create table if not exists roles(
  id uuid not null default uuid_generate_v4() primary key,
  role_name text not null,
  role_description text not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (role_name)
);
create or replace trigger update_roles_timestamp
  before update on roles for each row
  execute function update_timestamp();
--
-- role_permission_mappings table
create table if not exists role_permission_mappings(
  id uuid not null default uuid_generate_v4() primary key,
  role_id uuid not null references roles(id) on delete cascade,
  permission_id uuid not null references permissions(id) on delete cascade,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);
create or replace trigger update_role_permission_mappings_timestamp
  before update on role_permission_mappings for each row
  execute function update_timestamp();
--
-- user_role_mappings table
create table if not exists user_role_mappings(
  id uuid not null default uuid_generate_v4() primary key,
  user_id uuid not null references users(id) on delete cascade,
  role_id uuid not null references roles(id) on delete cascade,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (user_id, role_id)
);
create or replace trigger update_user_role_mappings_timestamp
  before update on user_role_mappings for each row
  execute function update_timestamp();
--
-- stickers table
create table if not exists stickers(
  id uuid not null default uuid_generate_v4() primary key,
  user_id uuid not null references users(id) on delete cascade,
  asset_id uuid not null references assets(id) on delete cascade,
  visibility asset_visibility not null default 'private' ::asset_visibility,
  friendly_name text not null,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);
create or replace trigger update_stickers_timestamp
  before update on stickers for each row
  execute function update_timestamp();
--
-- post_stickers table
create table if not exists post_stickers(
  id uuid not null default uuid_generate_v4() primary key,
  post_id uuid not null references posts(id) on delete cascade,
  sticker_id uuid not null references stickers(id) on delete cascade,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp
);
create or replace trigger update_post_stickers_timestamp
  before update on post_stickers for each row
  execute function update_timestamp();
--
-- create user_permission_mappings table
create table if not exists user_permission_mappings(
  id uuid not null default uuid_generate_v4() primary key,
  user_id uuid not null references users(id) on delete cascade,
  permission_id uuid not null references permissions(id) on delete cascade,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (user_id, permission_id)
);
create or replace trigger update_user_permission_mappings_timestamp
  before update on user_permission_mappings for each row
  execute function update_timestamp();
--
-- create eversql recommended index to optimize role permissions select queries
create index role_mappings_idx_role_id_permission_id on "jen"."role_permission_mappings"("role_id", "permission_id");
--
-- create initial permissions
insert into jen.permissions(permission_name, permission_description)
  values
    -- profiles
('profile:get', 'Allow a user to view their profile'),
('profile:edit', 'Allow a user to edit their profile'),
    -- tags
    -- ('tags:get', 'Allow a user to view tags'),
('tags:create', 'Allow a user to create new tags'),
('tags:edit', 'Allow a user to edit existing tags'),
('tags:delete', 'Allows a user to delete tags'),
    -- spaces
    -- ('spaces:get', 'Allow a user to view spaces'),
('spaces:create', 'Allow a user to create new spaces'),
('spaces:edit', 'Allow a user to edit existing spaces'),
('spaces:delete', 'Allow a user to delete existing spaces'),
    -- stickers
('stickers:get', 'Allow a user to view available stickers'),
('stickers:create', 'Allow a user to create new stickers'),
('stickers:edit', 'Allow a user to edit existing stickers'),
('stickers:delete', 'Allow a user to delete existing stickers'),
    -- posts
    -- ('posts:get', 'Allow a user to view spaces'),
('posts:create', 'Allow a user to create new posts'),
('posts:edit', 'Allow a user to edit their existing posts'),
('posts:delete', 'Allow a user to delete their existing posts'),
('posts:likes:create', 'Allow a user to like posts'),
('posts:likes:get-count', 'Allow a user to view the amount of likes on a post'),
('posts:likes:delete', 'Allow a user to remove their like from a post'),
    -- comment
('comments:get', 'Allow a user to view comments'),
('comments:create', 'Allow a user to create new comments'),
('comments:edit', 'Allow a user to edit their existing comments'),
('comments:delete', 'Allow a user to delete their existing comments'),
('comments:likes:get-count', 'Allow a user to view the number of likes on a comment'),
('comments:likes:delete', 'Allow a user to remove their like from a comment'),
('comments:likes:create', 'Allow a user to start or continue a comment thread');
--
-- create initial roles
insert into jen.roles(role_name, role_description)
  values
    -- default
('mocha-default', 'Default roles for a new user. This role is a container for several readonly permissions.'),
    -- admin
('mocha-admin', 'Role containing permissions for admin users.');
--
-- create utility to get the id of a role by its name
create or replace function get_role_id(rname text)
  returns uuid
  as $$
declare
  role_id uuid;
begin
  role_id :=(
    select
      id
    from
      jen.roles
    where
      role_name = rname);
  return role_id;
end;
$$
language plpgsql;
--
-- create utility to get id of a permission by its name
create or replace function get_permission_id(pname text)
  returns uuid
  as $$
declare
  permission_id uuid;
begin
  permission_id :=(
    select
      id
    from
      jen.permissions
    where
      permission_name = pname);
  return permission_id;
end;
$$
language plpgsql;
--
-- create role permission mappings
insert into role_permission_mappings(role_id, permission_id)
  values
    --
    -- default
(get_role_id('mocha-default'), get_permission_id('posts:likes:create')),
(get_role_id('mocha-default'), get_permission_id('posts:likes:get-count')),
(get_role_id('mocha-default'), get_permission_id('posts:likes:delete')),
(get_role_id('mocha-default'), get_permission_id('comments:get')),
(get_role_id('mocha-default'), get_permission_id('comments:create')),
(get_role_id('mocha-default'), get_permission_id('comments:edit')),
(get_role_id('mocha-default'), get_permission_id('comments:delete')),
(get_role_id('mocha-default'), get_permission_id('comments:likes:create')),
(get_role_id('mocha-default'), get_permission_id('comments:likes:get-count')),
    --
    -- admin
(get_role_id('mocha-admin'), get_permission_id('profile:get')),
(get_role_id('mocha-admin'), get_permission_id('profile:edit')),
    --
(get_role_id('mocha-admin'), get_permission_id('tags:create')),
(get_role_id('mocha-admin'), get_permission_id('tags:edit')),
(get_role_id('mocha-admin'), get_permission_id('tags:delete')),
    --
(get_role_id('mocha-admin'), get_permission_id('spaces:create')),
(get_role_id('mocha-admin'), get_permission_id('spaces:edit')),
(get_role_id('mocha-admin'), get_permission_id('spaces:delete')),
    --
(get_role_id('mocha-admin'), get_permission_id('stickers:get')),
(get_role_id('mocha-admin'), get_permission_id('stickers:create')),
(get_role_id('mocha-admin'), get_permission_id('stickers:edit')),
(get_role_id('mocha-admin'), get_permission_id('stickers:delete')),
    --
(get_role_id('mocha-admin'), get_permission_id('posts:create')),
(get_role_id('mocha-admin'), get_permission_id('posts:edit')),
(get_role_id('mocha-admin'), get_permission_id('posts:delete')),
(get_role_id('mocha-admin'), get_permission_id('posts:likes:create')),
(get_role_id('mocha-admin'), get_permission_id('posts:likes:get-count')),
(get_role_id('mocha-admin'), get_permission_id('posts:likes:delete')),
    --
(get_role_id('mocha-admin'), get_permission_id('comments:get')),
(get_role_id('mocha-admin'), get_permission_id('comments:create')),
(get_role_id('mocha-admin'), get_permission_id('comments:edit')),
(get_role_id('mocha-admin'), get_permission_id('comments:delete')),
(get_role_id('mocha-admin'), get_permission_id('comments:likes:create')),
(get_role_id('mocha-admin'), get_permission_id('comments:likes:get-count')),
(get_role_id('mocha-admin'), get_permission_id('comments:likes:delete'));
--
-- some utility functions
create or replace function jen.get_role_name(rid uuid)
  returns text
  as $$
declare
  rname text;
begin
  rname :=(
    select
      role_name
    from
      jen.roles
    where
      id = rid);
  return rname;
end;
$$
language plpgsql;
--
create or replace function jen.get_permission_name(pid uuid)
  returns text
  as $$
declare
  pname text;
begin
  pname :=(
    select
      permission_name
    from
      jen.permissions
    where
      id = pid);
  return pname;
end;
$$
language plpgsql;
--
create or replace function jen.get_user_permissions(_user_id uuid)
  returns jen.permissions[]
  as $$
declare
  role_ids uuid[];
  declare loop_permissions jen.permissions[];
  declare permissions jen.permissions[] = '{}'::jen.permissions[];
  declare _role_id uuid;
begin
  select
    array (
      select
        role_id
      from
        jen.user_role_mappings
      where
        user_role_mappings.user_id = _user_id) into role_ids;
  raise notice 'role_ids: %', role_ids;
  foreach _role_id in array role_ids loop
    raise notice '%', _role_id;
    select
      array_agg(x)
    from (
      select
        permissions.id,
        permission_name,
        permission_description,
        permissions.created_at,
        permissions.updated_at
      from
        jen.role_permission_mappings
        join jen.permissions on permissions.id = role_permission_mappings.permission_id
          and role_permission_mappings.role_id = _role_id) x into loop_permissions;
    raise notice '%', loop_permissions;
    select
      array_cat(permissions, loop_permissions) into permissions;
  end loop;
  return permissions;
end;
$$
language plpgsql;
--
commit;

