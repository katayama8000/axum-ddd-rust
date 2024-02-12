# This is a Management Circle App for University.

## Stack

- Rust
- axum
- tokio
- Docker

## Design Pattern

- Domain Driven Design

## How to run

At first, you need to run the devcontainer
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

### fetch
```bash
curl -X GET http://127.0.0.1:3000/circle/{circle_id}
``` 

### update
```bash
curl -X PUT \
  -H "Content-Type: application/json" \
  -d '{
        "circle_name": "updated club name",
        "capacity": 15
      }' \
  http://127.0.0.1:3000/circle/{circle_id}
```

