## server

### setup

```bash
cp .env.mysql.dist .env.mysql
```

### up

```bash
docker compose up -d
docker compose exec -it server /bin/bash
```

or 

```bash
make server-up
``` 

### run

```bash
cargo run --bin main
```

<!-- ### watch

```bash
./watch.sh
``` -->

### down

```bash
make server-down
```
