# Rocket Resource Aggregator

This project is a simple data aggregator for resources on a LAN.

The HTTP server runs on Rocket 0.5 and uses sqlx+postgresql as the database.

The client side is Angular 12.

# Server quick start

These instructions don't serve the client-side files — only the server APIs.
To serve client-side files, see the frontend development(TBD) and build(TBD)
ections.

## Prerequisites

- Docker
- docker-compose
- rustc 1.52.1
- [sqlx CLI](https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md)

## Set up a database

There is a [docker-compose.yml](./docker-compose.yml) file that can be used to
quickly set up a postgres docker container. Run the commands below to create a
new database from scratch.

```
docker-compose up -d
export DATABASE_URL=postgres://default@localhost:6000
export PGPASSWORD=default
sqlx database drop
sqlx database create
sqlx migrate run
```

## Start the server

Using the same terminal as the command above, `cargo run` can be used to start
the Rocket HTTP server. Using the same terminal is important, because the
`export` commands set the necessary environment variables.

# Frontend quick start

## Prerequisites

- node (latest or an active LTS version)
- Angular CLI (Angular 12)

## Serve the files

```
cd public
ng serve
```

# Example APIs

## Get all resources

```sh
curl http://localhost:8000/resource
```

## Create a new resource

``` sh
# all fields except `other_fields` are required
curl http://localhost:8000/resource/new \
  -X POST \
  -H 'Content-Type: application/json' \
  --data '{"name":"hello","status":"hi","description":"hello world"}'
```

## Update an existing resource

``` sh
# all fields except `name` are optional

curl http://localhost:8000/resource \
  -X POST \
  -H 'Content-Type: application/json' \
  --data '{"name":"hello","description":"this is a useful description"}'

curl http://localhost:8000/resource \
  -X POST \
  -H 'Content-Type: application/json' \
  --data '{"name":"hello","status":"unused"}'

curl http://localhost:8000/resource \
  -X POST \
  -H 'Content-Type: application/json' \
  --data '{"name":"hello","other_fields":{"hi":"hello"}}'
```

## Delete a resource

```sh
curl http://localhost:8000/resource \
  -X DELETE \
  -H 'Content-Type: application/json' \
  --data '{"name":"hello"}'
```
