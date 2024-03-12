create type role as enum
  ( 'admin'
  , 'exhibitor'
  , 'attendee'
  );

create table users
  ( "id"                uuid
                        primary key
                        default gen_random_uuid()
  , "role"              role
                        not null
                        default 'attendee'

  , "name"              text
                        not null
  , "mail"              text
                        not null

  , "created_at"        timestamp with time zone
                        not null
                        default now()
  , "modified_at"       timestamp with time zone
  );

create trigger prevent_update_of_created_at
before update on users for each row
execute function prevent_update_of_created_at();

create trigger update_modified_at
before update on users for each row
execute function update_modified_at();

insert into users
  ( "id"
  , "role"
  , "name"
  , "mail"
  )
values
  ( '00000000-0000-0000-0000-000000000000'
  , 'admin'
  , 'Admin'
  , 'conference@execut.nl'
  );

create table badges
  ( "id"                uuid
                        primary key
                        default gen_random_uuid()
  , "user_id"           uuid
                        not null
                        references users ( "id" )
                        on delete cascade
                        on update cascade

  , "badge"             uuid
                        not null
                        unique

  , "created_at"        timestamp with time zone
                        not null
                        default now()
  , "modified_at"       timestamp with time zone
  );

create trigger prevent_update_of_created_at
before update on badges for each row
execute function prevent_update_of_created_at();

create trigger update_modified_at
before update on badges for each row
execute function update_modified_at();

insert into badges
  ( "user_id"
  , "badge"
  )
values
  ( '00000000-0000-0000-0000-000000000000'
  , '00000000-0000-0000-0000-000000000000'
  );

create table tokens
  ( "id"                uuid
                        primary key
                        default gen_random_uuid()
  , "user_id"           uuid
                        not null
                        references users ( "id" )
                        on delete cascade
                        on update cascade

  , "token"             char(6)
                        not null

  -- The combination of `user_id` and `token` must be unique.
  , unique ( "user_id", "token" )

  -- The `is_used` column is used to prevent the same token from being used multiple times.
  , "is_used"           boolean
                        not null
                        default 'false'

  , "created_at"        timestamp with time zone
                        not null
                        default now()
  , "modified_at"       timestamp with time zone
  );

create trigger prevent_update_of_created_at
before update on tokens for each row
execute function prevent_update_of_created_at();

create trigger update_modified_at
before update on tokens for each row
execute function update_modified_at();

insert into tokens
  ( "user_id"
  , "token"
  )
values
  ( '00000000-0000-0000-0000-000000000000'
  , '000000'
  );
