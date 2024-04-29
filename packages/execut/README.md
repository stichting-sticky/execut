![exec(ut) logo](./icon.svg)

# exec(ut)

exec(ut) is a conference for and by IT students.

## Getting started

```bash
$ git clone git@github.com:stichtingsticky/execut.git

$ cd packages/execut/
```

## Usage

Ensure you have a Postgres database running. The easiest way to do so is using Docker. A `docker-compose.yml` file is included for your convenience.

```bash
$ docker compose up -d
```

```bash
$ cp .env.example .env
```

```bash
$ cargo run
```

Signing in as an admin

```bash
curl http://127.0.0.1:3000/v1/auth?token=000000 \
  -H 'Content-Type: application/json' \
  -d '{ "badge": "00000000-0000-0000-0000-000000000000" }'
```



## Routes

/health 
  GET -> health_check

/populate
  POST -> populate

/profiles
  GET -> get_profiles
x POST -> create_profile

/profiles/:id
  GET -> get_profile
x PUT -> update_profile
x PATCH -> modify_profile
  DELETE -> remove_profile

/badges
  GET -> get_badges
x POST -> create_badge

/badges/:id
  GET -> get_badge
x PUT -> update_badge
x PATCH -> modify_badge
x DELETE -> remove_badge

/scans
  GET -> get_scans
x DELETE -> reset_scans

/scans/:subject
  POST -> scan_badge
  GET -> get_scan
x PUT -> update_scan
x PATCH -> modify_scan
x DELETE -> remove_scan
  
/auth
  POST -> auth

## License

Copyright 2023 - 2024 Sem van Nieuwenhuizen. All Rights Reserved. This project is licensed under the terms of the `MIT` license. You can check out the full license [here](./LICENSE).
