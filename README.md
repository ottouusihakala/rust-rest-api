# Simple REST API written in Rust
Uses actix-web and MongoDB. Currently only works as a very simple Todo app backend.

## Running the development environment
1. Run `docker compose up` to start MongoDB
2. Run `cargo run` to start app
3. App should now be available at localhost:8080

## API

### GET /todo/{id}
Returns todo item with the given ID. Needs to be a valid ObjectId.

### POST /todo
Creates a new todo item. Message body needs to contain `task` and `completed`.
```
{
    "task": "A task that must be done",
    "completed": false
}
```