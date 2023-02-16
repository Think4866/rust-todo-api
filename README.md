# Todo API in Rust

This is a simple todo API written in Rust. It uses the [Actix](https://actix.rs/) web framework.
The application is a simple todo list that can be used to create, read, update and delete todos.

## Running the application

To run the application, you need to have [Rust](https://www.rust-lang.org/) installed on your machine.
You can then run the application using the following command:

```sh
cargo run
```

Make sure to set the `DATABASE_URL` environment variable to the database URL you want to use.

## Environment variables

The environment variables that can be set are:

- `DATABASE_URL`: The database URL to use. For example `postgres://postgres:postgres@localhost:5432/postgres`.
- `PORT`: The port to listen on. Recommended default to `8000`.
- `HOST`: The host to listen on. Recommended default is `localhost`.

### Development environment variables

- `POSTGRES_USER`: The username to use when bootstraping the Postgres database via Docker (for local development).
- `POSTGRES_PASSWORD`: The password to use when bootstraping the Postgres database via Docker (for local development).
- `POSTGRES_DB`: The database to use when connecting to the Postgres database.

### Docker

The application's database dependency can be run in a Docker container. To do so, you need to have Docker installed on your machine.
You can then run the application using the following command:

```sh
docker-compose up
```

By default, the Postgres database is exposed on port `5432`.

## Migrations

The application uses [SeaORM](https://www.sea-ql.org/SeaORM/) to manage migrations.
To run the migrations, you need to have [Rust](https://www.rust-lang.org/) installed on your machine.
Details about the available commands can be found in the [SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/migration/).
You should run migration commands from the `migration` directory.

## Testing

Once the API is running locally, you can test it using the following cURL commands.
Automated tests are not currently implemented.

### Create a todo

```sh
curl --request POST \
  --url http://localhost:8000/todos \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test",
	"completed": false
}'
```

### Get all todos

```sh
curl --request GET \
  --url http://localhost:8000/todos
```

### Delete a todo

```sh
curl --request DELETE \
  --url http://localhost:8000/todos/1
```

### Update a todo

```sh
curl --request PUT \
  --url http://localhost:8000/todos/1 \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Updated Title",
	"completed": true
}'
```

### Get a todo

```sh
curl --request GET \
  --url http://localhost:8000/todos/1
```
