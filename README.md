# Rocket Resource Aggregator

This project is a basic data aggregator for resources on a LAN.

The LAN wording is important because there are no security considerations done
for the code.

The HTTP server runs on Rocket 0.5 and uses sqlx+postgresql as the database.

The client side is Angular 12.

For the most part, this is a learning project for Rocket and Angular, though it
also fulfills a personal need.

# Demo video

[Click here](https://user-images.githubusercontent.com/6634136/128622429-82043e9c-3241-48f7-aa94-9f98bc6ec10a.mp4)

# Server quick start

These instructions don't serve the client-side files — only the server APIs
which will allow CORS requests from the default Angular `ng serve` address.

To serve client-side files, see the [front-end](#Frontend-quick-start) and
[build](#Build) sections.

## Dependencies

- Docker
- docker-compose
- rustc `>= 1.52.1`
- [sqlx CLI](https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md)

## Set up a database

There is a [docker-compose.yml](./docker-compose.yml) file that can be used to
quickly set up a postgres docker container. Run the command below from the root
folder of the git repository:

```
docker-compose up -d
```

## Start the server

Use the commands below to start the Rocket HTTP server:

```
export DATABASE_URL=postgres://default@localhost:6000
export PGPASSWORD=default
export SQLX_OFFLINE=true
cargo run --features dev_cors
```

# Frontend quick start

## Dependencies

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
# The fields `name` and `description` are required
curl http://localhost:8000/resource/new \
  -X POST \
  -H 'Content-Type: application/json' \
  --data '{"name":"hello","description":"hello world"}'
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

# Build

These are the instructions to generate a `.tar.gz` file containing all that is
needed to run the application on another device. The other device needs to have
docker and docker-compose installed.

## Dependencies

Building depends on both the [server](#Dependencies) and [front-end](#Dependencies-1) dependencies.

## Commands

```sh
# create the archive
bash build.sh

# move the bundle
rsync -avz resource-aggregator.tar.gz my_ssh_alias:~

# ssh into the other device.
ssh my_ssh_alias

tar xf resource-aggregator.tar.gz
cd resource-aggregator
docker-compose up -d

# usually need sudo to serve on port 80
sudo bash -c '
export DATABASE_URL=postgres://default@localhost:6000
export PGPASSWORD=default
export ROCKET_ADDRESS=0.0.0.0
export ROCKET_PORT=80
./server'

```
