create table scans
  ( "id"                uuid
                        primary key
                        default gen_random_uuid()
  , "initiator_id"      uuid
                        not null
                        references users ( "id" )
                        on delete cascade
                        on update cascade
  , "subject_id"        uuid
                        not null
                        references users ( "id" )
                        on delete cascade
                        on update cascade

  -- Initiator can only scan a subject once
  , unique ( "initiator_id", "subject_id" )

  -- The initiator or subject cannot be an admin
  -- , check ( "initiator_id" != '00000000-0000-0000-0000-000000000000' )
  -- , check ( "subject_id"   != '00000000-0000-0000-0000-000000000000' )

  -- Initiators can not scan their own badge
  , check ( "initiator_id" != "subject_id" )

  , "is_expunged"       boolean
                        not null
                        default 'false'

  , "created_at"        timestamp with time zone
                        not null
                        default now()
  , "modified_at"       timestamp with time zone
  );

create trigger prevent_update_of_created_at
before update on scans for each row
execute function prevent_update_of_created_at();

create trigger update_modified_at
before update on scans for each row
execute function update_modified_at();
