# Management Circle App for University

## Tech Stack

- Rust
- axum
- tokio
- Docker

## Design Pattern

- Domain Driven Design

## How to run
1. Copy the `.env.dist` file to `.env`.
  ```bash
  cp .env.dist .env
  ```
2. First, start the database container. Refer to the [docs](DB.md) for more details.
3. Next, run the devcontainer.
4. Finally, start the server with the following command:

```bash
cargo run --bin main
```

<!-- Alternatively, you can start the server in watch mode with:

```bash
./watch.sh
``` -->


### create 
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
        "circle_name": "music club",
        "capacity": 10,
        "owner_name": "John Lennon",
        "owner_age": 21,
        "owner_grade": 3,
        "owner_major": "Music"
      }' \
  http://127.0.0.1:3000/circle
```

### find
```bash
curl -X GET http://127.0.0.1:3000/circle/{circle_id}
``` 

### update
```bash
curl -X PUT \
  -H "Content-Type: application/json" \
  -d '{
        "circle_name": "football club",
        "capacity": 15
      }' \
  http://127.0.0.1:3000/circle/{circle_id}
```

