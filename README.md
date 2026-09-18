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
4. Copy DB env templates:
  - `cp .env.mysql.dist .env.mysql`
  - `cp .env.tidb.dist .env.tidb` (if you use TiDB)
  - Add a JWT signing secret to the env file you use. It must be at least 32 characters, and
    the server refuses to start without it:

    ```bash
    echo "JWT_SECRET=$(openssl rand -base64 32)" >> .env.mysql
    ```

    `ACCESS_TOKEN_TTL_SECONDS` is optional and defaults to `3600`.
5. Open the command palette (Command+Shift+P) and select "Dev Containers: Open Folder in Container..."
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

## Authentication

Sign-up and sign-in return a JWT access token. Protected endpoints expect it in the
`Authorization` header as `Bearer <token>`.

### sign up
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
        "email": "user@example.com",
        "password": "correct horse battery staple"
      }' \
  http://127.0.0.1:3000/auth/sign-up
```

Returns `201 Created`, or `409 Conflict` if the email is already registered. Passwords must be
at least 8 characters.

### sign in
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
        "email": "user@example.com",
        "password": "correct horse battery staple"
      }' \
  http://127.0.0.1:3000/auth/sign-in
```

Returns `401 Unauthorized` for both an unknown email and a wrong password, so the response
cannot be used to find out which addresses are registered.

### call a protected endpoint
```bash
curl -X GET \
  -H "Authorization: Bearer {access_token}" \
  http://127.0.0.1:3000/me
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

