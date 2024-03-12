create function prevent_update_of_created_at()
returns trigger as
$$
begin
  if new.created_at <> old.created_at then
    raise exception '`created_at` is immutable';
  end if;
  return new;
end
$$ language 'plpgsql';

create function update_modified_at()
returns trigger as
$$
begin
  new.modified_at = now();
  return new;
end
$$ language 'plpgsql';
