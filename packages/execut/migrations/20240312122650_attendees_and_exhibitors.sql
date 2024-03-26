create type unit as enum
  ( '()'
  );

create table attendees
  ( "user_id"           uuid
                        not null
  , "tag"               unit
                        not null
                        default '()'

  , primary key ( "user_id", "tag" )

  , "linkedin"          text
  , "study"             text
  , "degree"            text
  , "institution"       text
  , "graduation_year"   text

  , "created_at"        timestamp with time zone
                        not null
                        default now()
  , "modified_at"       timestamp with time zone
  );

create trigger prevent_update_of_created_at
before update on attendees for each row
execute function prevent_update_of_created_at();

create trigger update_modified_at
before update on attendees for each row
execute function update_modified_at();

create table exhibitors
  ( "user_id"           uuid
                        not null
  , "tag"               unit
                        not null
                        default '()'

  , primary key ( "user_id", "tag" )

  , "company"           text
                        not null

  , "created_at"        timestamp with time zone
                        not null
                        default now()
  , "modified_at"       timestamp with time zone
  );

create trigger prevent_update_of_created_at
before update on exhibitors for each row
execute function prevent_update_of_created_at();

create trigger update_modified_at
before update on exhibitors for each row
execute function update_modified_at();

alter table users
add column
  "attendee_tag" unit,
add foreign key ( "id", "attendee_tag" )
  references attendees ( "user_id", "tag" )
  deferrable
  initially deferred,
add column
  "exhibitor_tag" unit,
add foreign key ( "id", "exhibitor_tag" )
  references attendees ( "user_id", "tag" )
  deferrable
  initially deferred,
add check
  ( ( "role" = 'admin' and "attendee_tag" is null and "exhibitor_tag" is null ) or
    ( "role" = 'exhibitor' and "attendee_tag" is null ) or
    ( "role" = 'attendee'  and "exhibitor_tag" is null ) );

alter table attendees
add foreign key ( "user_id" )
  references users ( "id" )
  on delete cascade
  on update cascade;

alter table exhibitors
add foreign key ( "user_id" )
  references users ( "id" )
  on delete cascade
  on update cascade;
