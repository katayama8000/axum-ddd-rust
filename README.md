# This is a Management  Circle App for University

## Stack

- Rust
- axum
- tokio
- Docker

## Design Pattern

- Domain Driven Design

## How to run

You need to run the devcontainer first.

Then you can run the following command to start the server.


```bash
cargo run
```

or you can run the following command to start the server with watch mode.

```bash
./watch.sh
```


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

