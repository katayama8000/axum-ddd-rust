## server

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
cargo run
```

### watch

```bash
./watch.sh
```

### down

```bash
make server-down
```
