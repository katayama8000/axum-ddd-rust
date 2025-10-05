# Management Circle App for University

## Tech Stack

- Rust
- axum
- tokio
- Docker

## Design Pattern

- Domain Driven Design

## How to run
1. Clone this repository.
  `git clone https://github.com/katayama8000/axum-ddd-rust.git`
2. Open the repository in VSCode.
3. Install the "Dev Containers" extension in VSCode if you haven't already.
4. Copy .env.dist to .env
5. Open the command palette (Command+Shift+P) and select "Remote-Containers: Open Folder in Container..."
6. Select the cloned repository folder.
7. Wait for the container to build and start. This may take a few minutes.
8. Once the container is running, open a terminal in VSCode.
9. Run the following command to start the server:

```bash
cargo run --bin main
```
or you can use the watch script to auto-restart the server on code changes:

```bash
./watch.sh
```

### check version to see if the server is running
```bash
curl -X GET http://127.0.0.1:3000/version
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

